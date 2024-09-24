from pathlib import Path
import shutil
import sys
import os
import subprocess
import argparse
import tempfile
import time
import requests
from typing import Optional, Tuple


def find_gpu_device_id(
    host: str, port: int, vendor_id: int
) -> Optional[Tuple[str, int]]:
    res = requests.get(f"http://{host}:{port}/devices").json()

    for entry in res:
        if entry["vendor_id"] == vendor_id:
            print(entry)
            return (entry["device_name"], entry["device_id"])

    return None


def grab_compiler_args(glsl_file_path: Path, target_directive: str) -> str:
    with glsl_file_path.open() as f:
        lines = f.readlines()

    for line in lines:
        if not line.startswith("//"):
            continue
        line = line[2:].strip()
        parts = line.split(":")
        directive = parts[0].strip()
        arg = parts[-1].strip()

        if directive == target_directive:
            return arg

    return ""


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("output_dir")
    parser.add_argument("shader_source_path")
    parser.add_argument("--type", type=str)
    parser.add_argument("--host", type=str, required=True)
    parser.add_argument("--port", type=int, default=9999)
    parser.add_argument("--device-id", type=int)
    parser.add_argument("--vendor-id", type=int, default=4318)
    parser.add_argument("--debug", action="store_true")

    args = parser.parse_args()

    output_dir = Path(args.output_dir)
    shader_source_path = Path(args.shader_source_path)
    shader_type = args.type
    host = args.host
    port = args.port
    device_id = args.device_id
    vendor_id = args.vendor_id
    debug = args.debug

    if shader_type is None:
        if shader_source_path.name.endswith(".glsl"):
            shader_type = "glsl"
        elif shader_source_path.name.endswith(".asm"):
            shader_type = "spv-asm"
        elif shader_source_path.name.endswith(".spv"):
            shader_type = "spv"
        else:
            sys.stderr.write("Unknown shader type, please provide it via --type\n")
            return 1

    if device_id is None:
        device_search_result = find_gpu_device_id(host, port, vendor_id)
        if device_search_result is not None:
            (device_name, device_id) = device_search_result
            print(f'Using "{device_name}" (0x{device_id:x})')
        else:
            sys.stderr.write("Couldn't find an NVIDIA GPU on the target host\n")
            return 1

    if not debug:
        temporary_directory = tempfile.TemporaryDirectory(prefix="shader-comp-")
        temporary_directory_path = Path(temporary_directory.name)
    else:
        temporary_directory_path = output_dir

    compiler_extensions = ""
    compiler_shader_flags = ""
    spv_file = None

    output_dir.mkdir(parents=True, exist_ok=True)

    if shader_type == "glsl":
        compiler_extensions = grab_compiler_args(
            shader_source_path, "compiler_extensions"
        )
        compiler_shader_flags = grab_compiler_args(
            shader_source_path, "compiler_shader_flags"
        )

        spv_file = temporary_directory_path.joinpath("output.spv")
        args = [
            "glslangValidator",
            "-g0",
            "--target-env",
            "vulkan1.3",
            str(shader_source_path),
            "-o",
            str(spv_file),
        ]
        subprocess.check_call(args)
    elif shader_type == "spv-asm":
        spv_file = temporary_directory_path.joinpath("output.spv")
        args = [
            "spirv-as",
            "--target-env",
            "vulkan1.3",
            str(shader_source_path),
            "-o",
            str(spv_file),
        ]
        subprocess.check_call(args)
    elif shader_type == "spv":
        spv_file = shader_source_path

    assert spv_file is not None

    # Now let's send the file and let the Rust side output everything
    args = [
        "./target/debug/nvshaderdump",
        "remote",
        "--hostname",
        host,
        "--port",
        str(port),
        "--vendor-id",
        str(vendor_id),
        "--device-id",
        str(device_id),
        str(spv_file),
        "--output-directory",
        str(temporary_directory_path),
        "--extensions",
        compiler_extensions,
        "--shader-flags",
        compiler_shader_flags,
    ]
    res = subprocess.call(args)

    while res != 0:
        print("nvshaderdump failed retrying in 5s")
        time.sleep(5)
        res = subprocess.call(args)

    source_file_name = shader_source_path.stem

    if temporary_directory_path != output_dir:
        FILE_MAPPING = {
            "shader_header.bin": ".hdr",
            "shader_data.bin": ".code",
            "mesh_shader_header_gs.bin": ".gs_hdr",
        }

        for src_entry, suffix in FILE_MAPPING.items():
            src_file_path = temporary_directory_path.joinpath(src_entry)
            dst_file_path = output_dir.joinpath(f"{source_file_name}{suffix}")

            if src_file_path.exists() and os.stat(src_file_path).st_size != 0:
                shutil.copy(src_file_path, dst_file_path)

    return 0


if __name__ == "__main__":
    sys.exit(main())
