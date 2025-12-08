from utility import *

def pparse(filestr):
    ds = parse(filestr, atoms, show=False)[:-1]
    symbols = re.split(r"\s+", open(f"input/{filestr}").read().splitlines()[-1].strip())
    return ds, symbols

def part1(data, sym):
    out = [0] * len(data[0])
    for i, row in enumerate(data):
        for col, (s, val) in enumerate(zip(sym, row)):
            # print(col, s, val)
            if s == "*":
                if i == 0:
                    out[col] = val
                else:
                    out[col] *= val
            else:
                out[col] += val
    print(sum(out))

def part2(data):
    pass

ds, dss = pparse("06s")
dd, dds = pparse("06")

part1(ds, dss)
part1(dd, dds)
print()

