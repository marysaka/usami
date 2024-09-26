import argparse
from pathlib import Path
import sys
from typing import Any, Dict, List, Optional, Tuple


U32_MAX_VALUE = 0xFFFFFFFF


def select_byte(value: int, byte_index: int) -> int:
    return value >> (byte_index * 8) & 0xFF


def parse_pred(pred: str) -> Tuple[int, bool, str]:
    if pred[0] != "@":
        return (7, False, pred)

    idx = 1
    invert_pred = False

    if pred[idx] == "!":
        invert_pred = True
        idx += 1

    assert pred[idx] == "P"

    if pred[idx + 1] == "T":
        pred_idx = 7
    else:
        pred_idx = int(pred[idx + 1])

    idx += 3

    return (pred_idx, invert_pred, pred[idx:])


class InstrInfo(object):
    raw: str

    name: str
    args: List[str]
    flags: List[str]

    pred_idx: int
    invert_pred: bool

    def __init__(
        self,
        raw: str,
        name: str,
        args: List[str],
        flags: List[str],
        pred_idx: int,
        invert_pred: bool,
    ) -> None:
        self.raw = raw
        self.name = name
        self.args = args
        self.flags = flags
        self.pred_idx = pred_idx
        self.invert_pred = invert_pred

    @staticmethod
    def parse_instr(instr_str: str) -> "InstrInfo":
        (pred_idx, invert_pred, instr_rest) = parse_pred(instr_str)
        instr_name = instr_rest.split(" ")[0].split(".")[0].strip()
        instr_args = []
        instr_flags = []

        instr_flags_start = instr_rest.find(".")

        if instr_flags_start != -1:
            instr_flags_end = instr_rest.find(" ")

            instr_flags_raw = instr_rest[instr_flags_start:instr_flags_end]

            for tmp in instr_flags_raw.split("."):
                if len(tmp) == 0:
                    continue

                instr_flags.append(tmp)

        instr_args_start = instr_rest.find(" ")

        if instr_args_start != -1:
            instr_args_raw = instr_rest[instr_args_start:].strip()
            for tmp in instr_args_raw.split(" "):
                tmp = tmp.replace(",", "")
                if len(tmp) == 0:
                    continue

                instr_args.append(tmp)

        return InstrInfo(
            instr_str, instr_name, instr_args, instr_flags, pred_idx, invert_pred
        )


def do_imad(ctx: "EmulatorContext", info: InstrInfo):
    if "X" in info.flags and ctx.debug:
        sys.stderr.write(f"WARN: Skipping unsupported IMAD.X\n")
        return

    src0 = ctx.read_from_src(info.args[1])
    src1 = ctx.read_from_src(info.args[2])
    src2 = ctx.read_from_src(info.args[3])

    res = src0 * src1 + src2
    ctx.set_dst(info.args[0], res)


def do_iadd3(ctx: "EmulatorContext", info: InstrInfo):
    if "X" in info.flags and ctx.debug:
        sys.stderr.write(f"WARN: Skipping unsupported IADD3.X\n")
        return

    src_idx = 1

    # TODO: Overflow support
    # Skip overflow low predicate if present
    if parse_src_text(info.args[src_idx])["type"] == "pred":
        src_idx += 1

    # Skip overflow high predicate if present
    if parse_src_text(info.args[src_idx])["type"] == "pred":
        src_idx += 1

    src0 = ctx.read_from_src(info.args[src_idx + 0])
    src1 = ctx.read_from_src(info.args[src_idx + 1])
    src2 = ctx.read_from_src(info.args[src_idx + 2])

    res = src0 + src1 + src2
    ctx.set_dst(info.args[0], res)


def do_isetp(ctx: "EmulatorContext", info: InstrInfo):
    # XXX: Partially implemented

    cmp_op = info.flags[0]
    int_type = info.flags[1]
    assert int_type == "U32" or int_type == "I32"
    set_op = info.flags[2]

    src0 = ctx.read_from_src(info.args[2])
    src1 = ctx.read_from_src(info.args[3])

    if cmp_op == "EQ":
        res = src0 == src1
    elif cmp_op == "NE":
        res = src0 != src1
    elif cmp_op == "LT":
        res = src0 < src1
    elif cmp_op == "LE":
        res = src0 <= src1
    elif cmp_op == "GT":
        res = src0 > src1
    elif cmp_op == "GE":
        res = src0 >= src1
    else:
        raise Exception(f"Unsupported cmp_op: {cmp_op}")

    # XXX: is it right?
    acc = ctx.read_from_src(info.args[4])

    if set_op == "AND":
        p = res & acc
        q = (not res) & acc
    elif set_op == "OR":
        p = res | acc
        q = (not res) | acc
    elif set_op == "XOR":
        p = res ^ acc
        q = (not res) ^ acc
    else:
        raise Exception(f"Unsupported set_op: {set_op}")

    ctx.set_dst(info.args[0], p)
    ctx.set_dst(info.args[1], q)


def do_mov(ctx: "EmulatorContext", info: InstrInfo):
    src0 = ctx.read_from_src(info.args[1])
    ctx.set_dst(info.args[0], src0)


def do_s2r(ctx: "EmulatorContext", info: InstrInfo):
    src0 = ctx.spr_read_callback(info.args[1])
    ctx.set_dst(info.args[0], src0)


def do_cs2r(ctx: "EmulatorContext", info: InstrInfo):
    if info.args[1] != "SRZ" and ctx.debug:
        sys.stderr.write(f"WARN: Skipping unsupported CS2R on non SRZ register\n")
        return

    ctx.set_dst(info.args[0], 0)


def do_nop(ctx: "EmulatorContext", info: InstrInfo):
    pass


def do_bra(ctx: "EmulatorContext", info: InstrInfo):
    # Ensure to take increment of IP here
    ctx.ip = ctx.read_from_src(info.args[0]) - 0x10


def do_exit(ctx: "EmulatorContext", info: InstrInfo):
    ctx.running = False


def do_ldg(ctx: "EmulatorContext", info: InstrInfo):
    load_size = 32

    if "128" in info.flags:
        load_size = 128
    elif "64" in info.flags:
        load_size = 64
    if "16" in info.flags:
        load_size = 16
    if "8" in info.flags:
        load_size = 8

    element_size = min(load_size, 32)
    src0 = ctx.read_from_src(info.args[1])

    for element_idx in range(load_size // 32):
        tmp = ctx.global_read_callback(src0 + element_idx * 4, element_size)
        ctx.set_dst(info.args[0], tmp, element_idx * 4)


def do_stg(ctx: "EmulatorContext", info: InstrInfo):
    store_size = 32

    if "128" in info.flags:
        store_size = 128
    elif "64" in info.flags:
        store_size = 64
    if "16" in info.flags:
        store_size = 16
    if "8" in info.flags:
        store_size = 8

    element_size = min(store_size, 32)
    src0 = ctx.read_from_src(info.args[0])

    for element_idx in range(store_size // 32):
        src1 = ctx.read_from_src(info.args[1], element_idx * 4)
        ctx.global_write_callback(src0 + element_idx * 4, src1, element_size)


def lop3_lut(x: int, y: int, z: int, lut: int) -> int:
    res = x & ~x

    if (lut & (1 << 0)) != 0:
        res = res | (~x & ~y & ~z)

    if (lut & (1 << 1)) != 0:
        res = res | (~x & ~y & z)

    if (lut & (1 << 2)) != 0:
        res = res | (~x & y & ~z)

    if (lut & (1 << 3)) != 0:
        res = res | (~x & y & z)

    if (lut & (1 << 4)) != 0:
        res = res | (x & ~y & ~z)

    if (lut & (1 << 5)) != 0:
        res = res | (x & ~y & z)

    if (lut & (1 << 6)) != 0:
        res = res | (x & y & ~z)

    if (lut & (1 << 7)) != 0:
        res = res | (x & y & z)

    return res


def shf(
    low_val: int,
    shift: int,
    high_val: int,
    high: bool = False,
    wrap: bool = False,
    is_right: bool = False,
) -> int:
    limit = 32

    n = shift & (limit - 1) if wrap else min(shift, limit)

    if not is_right:
        d = (high_val >> (limit - n)) | (low_val << n)
    else:
        d = (high_val << (limit - n)) | (low_val >> n)

    if high:
        return d >> 32

    return d & U32_MAX_VALUE


def lea(x: int, y: int, z: int, shift: int, is_high: bool) -> int:
    if is_high:
        if shift == 0:
            high = 0
        else:
            high = x >> (32 - shift)

        high = high | z << shift

        return high + y

    return (x << z) + y


def do_lop3(ctx: "EmulatorContext", info: InstrInfo):
    assert info.flags[0] == "LUT" and info.args[-1] == "!PT"

    src0 = ctx.read_from_src(info.args[1])
    src1 = ctx.read_from_src(info.args[2])
    src2 = ctx.read_from_src(info.args[3])
    lut = ctx.read_from_src(info.args[4])

    res = lop3_lut(src0, src1, src2, lut)
    ctx.set_dst(info.args[0], res)


def do_prmt(ctx: "EmulatorContext", info: InstrInfo):
    src0 = ctx.read_from_src(info.args[1])
    src1 = ctx.read_from_src(info.args[2])
    src2 = ctx.read_from_src(info.args[3])

    select0 = (src1 >> 0) & 0xF
    select1 = (src1 >> 4) & 0xF
    select2 = (src1 >> 8) & 0xF
    select3 = (src1 >> 12) & 0xF

    tmp = src0 | src2 << 32
    res = (
        select_byte(tmp, select0)
        | select_byte(tmp, select1) << 8
        | select_byte(tmp, select2) << 16
        | select_byte(tmp, select3) << 24
    )
    ctx.set_dst(info.args[0], res)


def do_shf(ctx: "EmulatorContext", info: InstrInfo):
    src0 = ctx.read_from_src(info.args[1])
    src1 = ctx.read_from_src(info.args[2])
    src2 = ctx.read_from_src(info.args[3])

    is_right = "R" in info.flags
    is_high = "HI" in info.flags
    is_wrap = "W" in info.flags
    res = shf(src0, src1, src2, is_high, is_wrap, is_right)
    ctx.set_dst(info.args[0], res)


def do_lea(ctx: "EmulatorContext", info: InstrInfo):
    src0 = ctx.read_from_src(info.args[1])
    src1 = ctx.read_from_src(info.args[2])
    src2 = ctx.read_from_src(info.args[3])
    src3 = None

    if len(info.args) >= 5:
        src3 = ctx.read_from_src(info.args[4])

    is_high = "HI" in info.flags
    res = lea(src0, src1, src2, src3, is_high)
    ctx.set_dst(info.args[0], res)


INSTRS_LUT = {
    "IMAD": do_imad,
    "IADD3": do_iadd3,
    "ISETP": do_isetp,
    "MOV": do_mov,
    "CS2R": do_cs2r,
    "S2R": do_s2r,
    "NOP": do_nop,
    "BRA": do_bra,
    "EXIT": do_exit,
    "LDG": do_ldg,
    "STG": do_stg,
    "LOP3": do_lop3,
    "PRMT": do_prmt,
    "SHF": do_shf,
    "LEA": do_lea,
}


def parse_common_param_text(param_text: str) -> Optional[Dict[str, object]]:
    if param_text[0] == "[" and param_text[-1] == "]":
        param_text = param_text[1:-1]

    if param_text[0] == "R":
        register_value_raw = param_text[1:].split(".")[0]
        register_value = 255

        if register_value_raw != "Z":
            register_value = int(register_value_raw, 10)

        return {"type": "gpr", "value": register_value}
    elif param_text[0] == "P":
        register_value_raw = param_text[1:]
        register_value = 7

        if register_value_raw != "T":
            register_value = int(register_value_raw, 10)

        return {"type": "pred", "value": register_value}

    return None


def parse_src_text(src_text: str) -> Optional[Dict[str, object]]:
    res = parse_common_param_text(src_text)

    if res:
        return res
    elif src_text.startswith("0x"):
        return {"type": "imm", "value": int(src_text, 16)}
    elif src_text.startswith("c["):
        parts = src_text[2:-1].split("][")
        return {"type": "cbuf", "idx": int(parts[0], 16), "offset": int(parts[1], 16)}

    raise Exception(src_text)

    return None


def parse_dst_text(dst_text: str) -> Optional[Dict[str, object]]:
    res = parse_common_param_text(dst_text)

    if res:
        return res

    raise Exception(dst_text)

    return None


class EmulatorContext(object):
    gprs: List[int]
    preds: List[bool]

    ip: int
    shader_code: Dict[int, str]
    running: bool
    debug: bool
    cbuf_read_callback: Optional[Any]
    spr_read_callback: Optional[Any]
    global_read_callback: Optional[Any]
    global_write_callback: Optional[Any]
    # XXX: shared read/write callback
    # XXX: local read/write callback (Do we care? if not we only need the size here)

    def __init__(
        self,
        shader_code: Dict[int, str],
        cbuf_read_callback: Optional[Any] = None,
        spr_read_callback: Optional[Any] = None,
        global_read_callback: Optional[Any] = None,
        global_write_callback: Optional[Any] = None,
    ) -> None:
        self.shader_code = shader_code
        self.cbuf_read_callback = cbuf_read_callback
        self.spr_read_callback = spr_read_callback
        self.global_read_callback = global_read_callback
        self.global_write_callback = global_write_callback
        self.gprs = [0] * 255
        self.preds = [0] * 7
        self.ip = 0
        self.running = False
        self.debug = False

    def read_gpr(self, idx: int) -> int:
        if idx == 255:
            return 0

        return self.gprs[idx]

    def set_gpr(self, idx: int, value: int):
        if idx == 255:
            return

        self.gprs[idx] = value

    def read_pred(self, idx: int) -> bool:
        if idx == 7:
            return True

        return self.preds[idx]

    def set_pred(self, idx: int, value: bool):
        if idx == 7:
            return

        self.preds[idx] = value

    def read_from_src_info(
        self, src_info: Dict[str, object], offset: int, element_size: int
    ) -> int | bool:
        assert src_info

        src_type = src_info["type"]

        if src_type == "gpr":
            return self.read_gpr(src_info["value"]) + offset
        elif src_type == "pred":
            assert offset == 0
            return self.read_pred(src_info["value"])
        elif src_type == "imm":
            assert offset == 0
            return src_info["value"]
        elif src_type == "cbuf":
            if self.cbuf_read_callback:
                return self.cbuf_read_callback(
                    src_info["idx"], src_info["offset"], offset, element_size
                )

            return 0

        raise Exception(src_type)

    def set_dst_from_info(
        self, dst_info: Dict[str, object], value: int, offset: int, element_size: int
    ):
        assert dst_info

        dst_type = dst_info["type"]

        if dst_type == "gpr":
            assert offset % 4 == 0 and element_size == 32
            self.set_gpr(dst_info["value"] + (offset // 4), value)
        elif dst_type == "pred":
            assert offset == 0 and element_size == 32
            self.set_pred(dst_info["value"], value)
        else:
            raise Exception(dst_type)

    def set_dst(self, dst_text, value: int, offset: int = 0, element_size: int = 32):
        if self.debug:
            print(f"{dst_text} = 0x{value:x}")

        return self.set_dst_from_info(
            parse_dst_text(dst_text), value, offset, element_size
        )

    def read_from_src(
        self, src_text: str, offset: int = 0, element_size: int = 32
    ) -> int | bool:
        return self.read_from_src_info(parse_src_text(src_text), offset, element_size)

    def exec_instr(self, instr: str):
        instr_info = InstrInfo.parse_instr(instr)
        if self.debug:
            print(f'Executing "{instr_info.raw}"')

        pred_sel = self.read_pred(instr_info.pred_idx)

        if instr_info.invert_pred:
            pred_sel = not pred_sel

        # Check if the instruction should be executed
        if not pred_sel:
            # sys.stderr.write(f"NOTE: skipping {instr_info.raw} because pred wasn't met\n")
            return

        if instr_info.name in INSTRS_LUT:
            INSTRS_LUT[instr_info.name](self, instr_info)
        else:
            sys.stderr.write(f"WARN: Unknown instruction {instr_info.name}, skipping\n")
            assert not self.debug

    def run(self, start_ip: int, debug: bool = False):
        self.ip = start_ip
        self.running = True
        self.debug = debug

        while self.running:
            instr = self.shader_code[self.ip]
            self.exec_instr(instr)
            self.ip += 0x10


def parse_assembly_from_file(path: Path) -> Dict[int, str]:
    with path.open() as f:
        lines = f.readlines()

    res = dict()

    for line in lines:
        line = line.strip()

        parts = line[2:].split("*/")

        instr_offset = int("0x" + parts[0], 16)
        raw_instr = parts[1][:-1].strip()
        res[instr_offset] = raw_instr

    return res


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("input_assembly_path")

    args = parser.parse_args()

    info = parse_assembly_from_file(Path(args.input_assembly_path))

    lane_id = 0
    stride = 0x10
    element = 0x20
    bindings = list()

    # Output buffer (set=0, binding=0)
    bindings.append([0] * 0x8000)

    # Stride buffer (set=0, binding=1)
    bindings.append([stride])

    # Unused buffer (set=0, binding=2)
    # bindings.append([])

    # Element buffer (set=0, binding=3)
    bindings.append([element])

    def read_cbuf_value(
        cbuf_idx: int, cbuf_offset: int, extra_offset: int, element_size: int
    ) -> int:
        assert cbuf_idx == 0
        assert element_size == 32

        cbuf_offset += extra_offset

        # set/binding mappings (XXX: max offset is unk, we assume 8 max for now)
        if cbuf_offset >= 0x30 and cbuf_offset <= 0xB0:
            binding_idx = (cbuf_offset - 0x30) // 0x10
            is_size = ((cbuf_offset - 0x30) % 0x10) == 0x8

            # We are assuming 1MiB max of data per bindings
            # And process fake address accordingly.
            max_buffer_size = 0x100000

            if not is_size:
                return max_buffer_size * (binding_idx + 1)

            if len(bindings) <= binding_idx:
                print(f"out of bound {binding_idx}")
                return 0

            return len(bindings[binding_idx]) * 0x4

        print(f"Unknown cbuf offset 0x{cbuf_offset:x}")

        return 0

    def read_special_reg_value(special_reg_name: str) -> int:
        if special_reg_name == "SR_LANEID":
            return lane_id

        return 0

    def read_global_value(address: int, element_size: int) -> int:
        assert element_size <= 32

        element_mask = (1 << element_size) - 1

        value = 0

        if address >= 0x100000 and address < 0x800000:
            binding_idx = (address // 0x100000) - 1
            offset = address % 0x100000

            aligned_offset = offset // 4
            value_shift = (offset % 4) * 8

            value = bindings[binding_idx][aligned_offset]
            value = value >> value_shift
        else:
            print(f"Unknown global read at address 0x{address:x}")

        return value & element_mask

    def write_global_value(address: int, value: int, element_size: int):
        print(f"write_global_value: 0x{address:x} = 0x{value:x} (size: {element_size})")
        pass

    ctx = EmulatorContext(
        info,
        read_cbuf_value,
        read_special_reg_value,
        read_global_value,
        write_global_value,
    )
    ctx.run(0, True)

    return 0


if __name__ == "__main__":
    sys.exit(main())
