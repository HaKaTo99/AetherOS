#!/usr/bin/env python3
import sys

if len(sys.argv) != 5:
    print("Usage: convert_raw_to_ppm.py <input.raw> <output.ppm> <width> <height>")
    sys.exit(2)

infile, outfile, width_s, height_s = sys.argv[1:5]
width = int(width_s)
height = int(height_s)

with open(infile, 'rb') as f:
    data = f.read()

expected = width * height * 4
if len(data) < expected:
    print(f"Warning: raw file shorter ({len(data)} bytes) than expected ({expected})")

with open(outfile, 'wb') as out:
    out.write(f"P6\n{width} {height}\n255\n".encode('ascii'))
    # raw is little-endian u32: bytes are B,G,R,A per pixel
    for i in range(width * height):
        idx = i * 4
        if idx + 2 < len(data):
            b = data[idx]
            g = data[idx + 1]
            r = data[idx + 2]
            out.write(bytes((r, g, b)))
        else:
            out.write(b"\x00\x00\x00")

print(f"Wrote {outfile}")
