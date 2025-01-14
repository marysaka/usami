import subprocess
import sys
import tempfile
from typing import Dict, Tuple, Optional

R0 = 0
R1 = 1
R2 = 2
R3 = 3
R4 = 4
RZ = 255

ALU_SRC0_ABS_BIT = 73
ALU_SRC0_NEG_BIT = 72

ALU_SRC1_ABS_BIT = 62
ALU_SRC1_NEG_BIT = 63

# NOTE: NOT ON HALF2
ALU_SRC2_ABS_BIT = 74
# NOTE: NOT ON HALF2
ALU_SRC2_NEG_BIT = 75

HALF2_SRC2_MODE_FLAG_BIT = 81
HALF2_FTZ_FLAG_BIT = 80
HALF2_SAT_FLAG_BIT = 77
# DNZ
HALF2_FMZ_FLAG_BIT = 76
HALF2_SRC0_MODE_FLAG_BIT = 74


# 0 = .BF16_V2, 1 = .F32
HALF2_F32_FLAG_BIT = 78
HALF2_SRC1_MODE_FLAG_BIT = 60

# .H1_H0
HALF2_SRC_MODE_FLAG_F16X2 = 0
# .F32 (gone on Ampere? still encode right but not on Ada)
HALF2_SRC_MODE_FLAG_F32 = 1
# .H0_H0
HALF2_SRC_MODE_FLAG_F16X2_LOW = 2
# .H1_H1
HALF2_SRC_MODE_FLAG_F16X2_HIGH = 3

# HADD2 start
HADD2_OPCODE = 0x030
# HADD2 end


# HFMA2 start
HFMA2_OPCODE = 0x031
# NOTE: SM86+
HFMA2_RELU_BIT = 79
# NOTE: SM86+
HFMA2_BF16_V2_BIT = 85
HFMA2_H0_NH1_BIT = 86

# NOTE: no neg abs on src2
# HFMUL end

# HMUL2 start
HMUL2_OPCODE = 0x032
# NOTE: broken imm32?
# HFMUL end

# HSET2 start
HSET2_OPCODE = 0x033

HSET2_PRED_SET_OP_BIT = 69
HSET2_BF_BIT = 71
HSET2_FLOAT_CMP_BIT = 76
HSET2_ACCUM_PRED_BIT = 87
# HSET2 end

# HSETP2 start
HSETP2_OPCODE = 0x034
HSETP2_PRED_SET_OP_BIT = 69
HSETP2_H_AND_BIT = 71
HSETP2_FLOAT_CMP_BIT = 76
HSETP2_DST0_PRED_BIT = 81
HSETP2_DST1_PRED_BIT = 84
HSETP2_ACCUM_PRED_BIT = 87
# HSETP2 end

# HMNMX2 start
HMNMX2_OPCODE = 0x040
HMNMX2_ACCUM_PRED_BIT = 87
HMNMX2_NAN_BIT = 81
HMNMX2_XORSIGN_BIT = 82
HMNMX2_BF16_V2_BIT = 85
# Support:
# - SRC0 and SRC1 neg/abs
# - FTZ
# - F32
# - Accum predicate
# HMNMX2 end


def clear_nvdisasm_output(stdout: str) -> str:
    return stdout.replace("/*0000*/", "").strip()


def disassemble_instr(instr: int, sm_ver: str) -> subprocess.CompletedProcess:
    # Extract the two QWORD
    instr_low = instr & 0xFFFFFFFFFFFFFFFF
    instr_high = instr >> 64

    raw_instr = bytearray()
    raw_instr += instr_low.to_bytes(length=8, byteorder="little")
    raw_instr += instr_high.to_bytes(length=8, byteorder="little")

    with tempfile.NamedTemporaryFile(delete_on_close=False) as fp:
        fp.write(raw_instr)
        fp.close()

        res = subprocess.run(
            ["nvdisasm", "-raw", "-b", sm_ver, fp.name],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

    return res


def disassemble_instr_to_str(instr: int, sm_ver: str) -> Optional[str]:
    res = disassemble_instr(instr, sm_ver)
    if res.returncode != 0:
        print(f"ERROR: nvdisasm failed on {instr:x}")
        return None

    return clear_nvdisasm_output(res.stdout.decode("utf-8"))


def encode_alu2_reg(
    alu_op: int, dst_reg: int, src_reg0: int, src_reg1: int = 0, src_reg2: int = 0
) -> int:
    alu_form = 0x1  # 2 regs

    instr = (
        alu_op & 0x1FF
        | (alu_form & 0x7) << 9
        | (dst_reg << 16)
        | (src_reg0 << 24)
        | (src_reg1 << 32)
        | (src_reg2 << 64)
    )

    return instr


def encode_alu2_imm32(
    alu_op: int, dst_reg: int, src_reg0: int, imm: int, is_imm_src1: bool = False
) -> int:
    alu_form = 0x2  # imm
    if is_imm_src1:
        alu_form = 0x4

    instr = (
        alu_op & 0x1FF
        | (alu_form & 0x7) << 9
        | (dst_reg << 16)
        | (src_reg0 << 24)
        | (imm << 32)
    )
    return instr


# IMMA START
IMMA_OPCODE = 0x37
IMMA_SRC1_ROW = 0 << 74
IMMA_SRC1_COL = 1 << 74
IMMA_SRC0_UNSIGNED = 0b00 << 76
IMMA_SRC0_SIGNED = 0b01 << 76
IMMA_SRC1_UNSIGNED = 0b00 << 78
IMMA_SRC1_SIGNED = 0b01 << 78
IMMA_SAT_BIT = 82
IMMA_SRC0_4BIT = 1 << 83
IMMA_SRC1_4BIT = 1 << 84

IMMA_MAT_TYPE_8816 = (0b0 << 75) | (0b00 << 85)
IMMA_MAT_TYPE_8832 = (0b0 << 75) | (0b01 << 85)
IMMA_MAT_TYPE_16816 = (0b0 << 75) | (0b10 << 85)
IMMA_MAT_TYPE_16864 = (0b0 << 75) | (0b11 << 85)
IMMA_MAT_TYPE_16832 = (0b1 << 75) | (0b10 << 85)
# IMMA END

# LDSM START
LDSM_OPCODE = 0x3B
LDSM_NUM_1 = 0b00 << 72
LDSM_NUM_2 = 0b01 << 72
LDSM_NUM_4 = 0b10 << 72
LDSM_FORM_M88 = 0b00 << 78
LDSM_FORM_MT88 = 0b01 << 78
LDSM_FORM_M816 = 0b10 << 78
LDSM_FORM_M832 = 0b11 << 78
# LDSM END

# MOVM START
MOVM_OPCODE = 0x3A
MOVM_FORM_MT88 = 0b00 << 78
MOVM_FORM_M832 = 0b01 << 78
MOVM_FORM_M864 = 0b10 << 78
# IMM20 aligned to 0x100
# MOVM END

instr = encode_alu2_reg(IMMA_OPCODE, R4, R2, 6, 8)
instr |= IMMA_MAT_TYPE_16832
instr |= IMMA_SRC1_COL
# instr = encode_alu2_imm32(LDSM_OPCODE, R4, R2, 0x100, True)
res = disassemble_instr_to_str(instr, "SM75")
print(res)
