use std::{
    fs::File,
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use argh::FromArgs;
use reqwest::multipart::Part;

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct Args {
    #[argh(subcommand)]
    subcommand: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Remote(RemoteSubCommand),
    Local(LocalSubCommand),
}

/// Remotely ask a shader dump and deserialize it.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "remote")]
struct RemoteSubCommand {
    /// the path to the SPIR-V file to send.
    #[argh(positional)]
    spirv_file_path: PathBuf,

    /// the hostname of the server to do the request.
    #[argh(option)]
    hostname: String,

    /// the port of the server to do the request.
    #[argh(option)]
    port: u32,

    /// the SPIR-V entrypoint.
    #[argh(option)]
    entrypoint: Option<String>,

    /// the vendor id.
    #[argh(option)]
    vendor_id: usize,

    /// the device id.
    #[argh(option)]
    device_id: usize,

    /// the optional output directory to store the shader code and header.
    #[argh(option)]
    output_directory: Option<PathBuf>,
}

/// Locally deserialize a NVVM container.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "local")]
struct LocalSubCommand {
    /// the path to the NVVM file to deserialize.
    #[argh(positional)]
    nvvm_file_path: PathBuf,

    /// the optional output directory to store the shader code and header.
    #[argh(option)]
    output_directory: Option<PathBuf>,
}

async fn get_shader_binary(args: &RemoteSubCommand) -> Vec<u8> {
    let url = format!("http://{}:{}/get_shader_binary", args.hostname, args.port);

    let mut spirv_data = Vec::new();

    let mut file = File::open(&args.spirv_file_path).unwrap();

    file.read_to_end(&mut spirv_data).unwrap();

    let form = reqwest::multipart::Form::new()
        .text(
            "entry_point",
            args.entrypoint.clone().unwrap_or("main".into()),
        )
        .text("vendor_id", args.vendor_id.to_string())
        .text("device_id", args.device_id.to_string())
        .part("file", Part::bytes(spirv_data));

    let response = reqwest::Client::new()
        .post(url)
        .multipart(form)
        .send()
        .await
        .expect("send");

    let mut result = Vec::new();

    if response.status() != 200 {
        let error_txt = response.text_with_charset("utf-8").await.unwrap();

        panic!("Server replied with error: {error_txt}")
    }

    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut result).unwrap();

    result
}

pub fn find_zstd_header(bin: &[u8]) -> Option<usize> {
    if bin.len() < 4 {
        return None;
    }

    let header = 0xfd2fb528_u32.to_ne_bytes();
    for i in 0..(bin.len() - 3) {
        if bin[i..(i + 4)] == header {
            return Some(i);
        }
    }
    None
}
pub fn get_zstd_shader_blob(bin: &[u8]) -> Option<Vec<u8>> {
    if let Some(offset) = find_zstd_header(bin) {
        Some(zstd::stream::decode_all(&bin[offset..]).unwrap())
    } else {
        None
    }
}

const FERMI_HDR_SIZE: usize = 96;
const TURING_HDR_SIZE: usize = 128;

#[derive(Debug)]
pub struct ShaderBlobInfo {
    pub offset: usize,
    pub size: usize,
}

pub fn find_shader_data_offsets(
    nvuc_container: &[u8],
) -> Option<(
    Option<ShaderBlobInfo>,
    ShaderBlobInfo,
    Option<ShaderBlobInfo>,
)> {
    let magic = u32::from_ne_bytes(nvuc_container[0..4].try_into().unwrap());
    assert!(magic == 0x6375564e);

    let section_count = u16::from_ne_bytes(nvuc_container[8..10].try_into().unwrap()) as usize;

    let nvuc_section_header = &nvuc_container[32..];

    let mut header_blob = None;
    let mut mesh_gs_header_blob = None;
    let mut code_blob = None;

    for section_index in 0..section_count {
        let section_header_offset = section_index * 32;
        let section_id = u16::from_ne_bytes(
            nvuc_section_header[section_header_offset..section_header_offset + 2]
                .try_into()
                .unwrap(),
        ) as usize;
        let section_size = u32::from_ne_bytes(
            nvuc_section_header[section_header_offset + 4..section_header_offset + 8]
                .try_into()
                .unwrap(),
        ) as usize;
        let section_offset = u64::from_ne_bytes(
            nvuc_section_header[section_header_offset + 8..section_header_offset + 16]
                .try_into()
                .unwrap(),
        ) as usize;

        if section_id == 0x2d {
            header_blob = Some(ShaderBlobInfo {
                offset: section_offset,
                size: section_size,
            })
        } else if section_id == 0x1 {
            code_blob = Some(ShaderBlobInfo {
                offset: section_offset,
                size: section_size,
            })
        } else if section_id == 0x4d {
            mesh_gs_header_blob = Some(ShaderBlobInfo {
                offset: section_offset,
                size: section_size,
            })
        }
    }

    if let Some(code_blob) = code_blob {
        return Some((header_blob, code_blob, mesh_gs_header_blob));
    }

    None
}

pub fn get_shader_data(nvuc_container: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let (header_info, shader_info, mesh_gs_header_info) =
        find_shader_data_offsets(nvuc_container).expect("Cannot find shader data offsets!");

    let header_data = if let Some(header_info) = header_info {
        // Get actual header size by detecting SPH version.
        let sph_version = (u16::from_ne_bytes(
            nvuc_container[header_info.offset..header_info.offset + 2]
                .try_into()
                .unwrap(),
        ) >> 5)
            & 0x1f;

        let hdr_expected_size = if sph_version < 4 {
            FERMI_HDR_SIZE
        } else {
            TURING_HDR_SIZE
        };

        // Sanity check that the values are in ranges.
        assert!(header_info.size == hdr_expected_size);
        assert!(header_info.offset + hdr_expected_size == shader_info.offset);

        nvuc_container[header_info.offset..header_info.offset + header_info.size].into()
    } else {
        Vec::new()
    };

    let mesh_gs_header_data = if let Some(header_info) = mesh_gs_header_info {
        nvuc_container[header_info.offset..header_info.offset + header_info.size].into()
    } else {
        Vec::new()
    };

    (
        header_data,
        nvuc_container[shader_info.offset..shader_info.offset + shader_info.size].into(),
        mesh_gs_header_data,
    )
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    let (nvvm_container, output_directory) = match args {
        Args {
            subcommand: SubCommandEnum::Remote(args),
        } => {
            let shader_binary = get_shader_binary(&args).await;

            if let Some(output_directory) = &args.output_directory {
                std::fs::create_dir_all(output_directory).unwrap();
            }

            if let Some(output_directory) = &args.output_directory {
                let mut file =
                    File::create(output_directory.join("shader_binary_raw.bin")).unwrap();
                file.write_all(&shader_binary).unwrap();
            }

            if let Some(mut data) = get_zstd_shader_blob(&shader_binary) {
                data.drain(0..8);

                (Some(data), args.output_directory)
            } else {
                eprintln!("ZSTD header not found!");

                (None, args.output_directory)
            }
        }

        Args {
            subcommand: SubCommandEnum::Local(args),
        } => {
            let mut data = Vec::new();

            let mut file = File::open(&args.nvvm_file_path).unwrap();

            file.read_to_end(&mut data).unwrap();

            data.drain(0..4);

            (Some(data), args.output_directory)
        }
    };

    if let Some(nvvm_container) = nvvm_container {
        if let Some(output_directory) = &output_directory {
            let mut file = File::create(output_directory.join("shader_zstd_dec.bin")).unwrap();
            file.write_all(&nvvm_container).unwrap();
        }

        let (shader_header_data, shader_binary_data, mesh_gs_header_data) =
            get_shader_data(&nvvm_container);

        if let Some(output_directory) = &output_directory {
            let mut file = File::create(output_directory.join("shader_header.bin")).unwrap();
            file.write_all(&shader_header_data).unwrap();

            let mut file = File::create(output_directory.join("shader_data.bin")).unwrap();
            file.write_all(&shader_binary_data).unwrap();

            let mut file =
                File::create(output_directory.join("mesh_shader_header_gs.bin")).unwrap();
            file.write_all(&mesh_gs_header_data).unwrap();
        }
    }
}
