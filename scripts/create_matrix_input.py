import numpy as np
import sys


ALIGN = 256

def create_matrix(row_size: int, colum_size: int):
    res = []

    for y in range(colum_size):
        row = []
        for _ in range(row_size):
            row.append(np.float16(y + 1))
        res.append(row)

    return res

def write_matrix(f, matrix):
    f.write(matrix.tobytes())

output_file = sys.argv[1]
m = int(sys.argv[2])
n = int(sys.argv[3])
k = int(sys.argv[4])

a = np.matrix(create_matrix(m, k), dtype=np.float16)
b = np.matrix(create_matrix(k, n), dtype=np.float16)
c = np.matrix(create_matrix(m, n), dtype=np.float16)

with open(output_file, "wb") as f:
    write_matrix(f, a)
    write_matrix(f, b)
    write_matrix(f, c)
