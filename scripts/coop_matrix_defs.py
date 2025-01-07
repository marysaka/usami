# MxNxK (A/B/C/D)
# MxK (A)
# KxN (B)
# MxN (C)
# MxN (D)

VK_TYPE_TO_GLSL_TYPE = {
    "FLOAT16": "float16_t",
    "FLOAT32": "float32_t",
    "FLOAT64": "float64_t",
    "UINT8": "uint8_t",
    "SINT8": "int8_t",
    "UINT16": "uint16_t",
    "SINT16": "int16_t",
    "UINT32": "uint32_t",
    "SINT32": "int32_t",
    "UINT64": "uint64_t",
    "SINT64": "int64_t",
}

VK_TYPE_TO_BYTE_SIZE = {
    "FLOAT16": 2,
    "FLOAT32": 4,
    "FLOAT64": 8,
    "UINT8": 1,
    "SINT8": 1,
    "UINT16": 2,
    "SINT16": 2,
    "UINT32": 4,
    "SINT32": 4,
    "UINT64": 8,
    "SINT64": 8,
}

MATRIX_USAGE_SHORT_NAME = {
    "gl_MatrixUseA": "use_a",
    "gl_MatrixUseB": "use_b",
    "gl_MatrixUseAccumulator": "use_c",
}

SUPPORTED_CFGS_SM75 = [
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 8,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 8,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 32,
        "a_type": "SINT8",
        "b_type": "SINT8",
        "c_type": "SINT32",
        "result_type": "SINT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 32,
        "a_type": "SINT8",
        "b_type": "SINT8",
        "c_type": "SINT32",
        "result_type": "SINT32",
        "saturating_accumulation": 1,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 32,
        "a_type": "UINT8",
        "b_type": "UINT8",
        "c_type": "UINT32",
        "result_type": "UINT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 32,
        "a_type": "UINT8",
        "b_type": "UINT8",
        "c_type": "UINT32",
        "result_type": "UINT32",
        "saturating_accumulation": 1,
    },
]

SUPPORTED_CFGS_SM86 = [
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 8,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT16",
        "result_type": "FLOAT16",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 16,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 8,
        "a_type": "FLOAT16",
        "b_type": "FLOAT16",
        "c_type": "FLOAT32",
        "result_type": "FLOAT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 32,
        "a_type": "UINT8",
        "b_type": "UINT8",
        "c_type": "UINT32",
        "result_type": "UINT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 16,
        "k_size": 32,
        "a_type": "SINT8",
        "b_type": "SINT8",
        "c_type": "SINT32",
        "result_type": "SINT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 32,
        "a_type": "UINT8",
        "b_type": "UINT8",
        "c_type": "UINT32",
        "result_type": "UINT32",
        "saturating_accumulation": 0,
    },
    {
        "m_size": 16,
        "n_size": 8,
        "k_size": 32,
        "a_type": "SINT8",
        "b_type": "SINT8",
        "c_type": "SINT32",
        "result_type": "SINT32",
        "saturating_accumulation": 0,
    },
]
