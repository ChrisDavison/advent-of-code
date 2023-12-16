from utility import *

SAMPLE = r""".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."""
DATA = Path("input/16").read_text()

d = DATA
P = []
for i, line in enumerate(d.splitlines()):
    n = []
    for j, ch in enumerate(line):
        n.append(ch)
    P.append(n)
tiles = np.array(P)


@dataclass
class Beam:
    x: int
    y: int
    xd: int
    yd: int

    def step(self, symbol):
        xd = self.xd
        yd = self.yd
        nb = lambda xd, yd: Beam(x=self.x+xd, y=self.y+yd, xd=xd, yd=yd)
        match symbol:
            case r'/':
                if xd:
                    return [nb(0, -xd)]
                else:
                    return [nb(-yd, 0)]
            case '\\' | '\\\\':
                if xd:
                    return [nb(0, xd)]
                else:
                    return [nb(yd, 0)]
            case r'|':
                if yd: # continue vertical
                    return [nb(0, yd)]
                else:
                    return [nb(0, -1), nb(0, 1)]
            case r'-':
                if xd: # continue horizontal
                    return [nb(xd, 0)]
                else:
                    return [nb(-1, 0), nb(1, 0)]
            case r'.':
                return [nb(xd, yd)]
            case x:
                print(f"Unexpected nextdir char {x = :}")

    def __str__(self):
        if self.xd != 0:
            xc = '←' if self.xd < 0 else '→'
            return f"{xc}{self.x},{self.y}"
        else:
            yc = '↑' if self.yd < 0 else '↓'
            return f"{self.x},{yc}{self.y}"


    def __hash__(self):
        return (self.x, self.y, self.xd, self.yd).__hash__()

    def within_bounds(self, dim):
        xlim, ylim = dim
        within_x = self.x >= 0 and self.x < xlim
        within_y = self.y >= 0 and self.y < ylim
        return within_x and within_y



# part1
timer()

beams = [Beam(x=0, y=0, xd=1, yd=0)]
tiles_energised = set()
loops = 10
dim = (len(P), (len(P[0])))
while beams:
    b = beams.pop()
    tiles_energised.add(b)
    try:
        symbol = P[b.y][b.x]
    except:
        continue
    for nb in b.step(symbol):
        oob = 'OOB' if not nb.within_bounds(dim) else ''
        # print(f"{b} into {symbol} => {nb}   {oob}")
        # if nb.x == 7 and nb.y == 0:
        #     print(f"{b} => {nb}")
        if nb.within_bounds(dim) and nb not in tiles_energised:
            # only count unique beams
            # and beams that are still in bounds
            beams.append(nb)


s = len(set((b.x, b.y) for b in tiles_energised))
for b in tiles_energised:
    tiles[b.y][b.x] = '#'

timer(f"Part 1: {s}")
pyperclip.copy(int(s))

if 0:
    # part2
    timer(reset=True)

    # ... do stuff ...

    res = 0
    timer(f"Part 2: {res}")
    pyperclip.copy(int(res))


