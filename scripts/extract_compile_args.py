import sys

glsl_file = sys.argv[1]
target_directive = sys.argv[2]

with open(glsl_file, "r") as f:
    lines = f.readlines()


for line in lines:
    if not line.startswith('//'):
        continue
    line = line[2:].strip()
    parts = line.split(':')
    directive = parts[0].strip()
    arg = parts[-1].strip()

    if directive == target_directive:
        sys.stdout.write(arg)
        sys.exit(0)