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

def energise(startbeam, grid):
    beams = [startbeam]
    tiles_energised = set()
    loops = 10
    dim = (len(grid), (len(grid[0])))
    while beams:
        b = beams.pop()
        tiles_energised.add(b)
        try:
            symbol = grid[b.y][b.x]
        except:
            continue
        for nb in b.step(symbol):
            if nb.within_bounds(dim) and nb not in tiles_energised:
                # only count unique beams
                # and beams that are still in bounds
                beams.append(nb)
    return len(set((b.x, b.y) for b in tiles_energised))

s = energise(Beam(x=0, y=0, xd=1, yd=0), P)
timer(f"Part 1: {s}")
pyperclip.copy(int(s))

def energise_every_beam(P):
    xlim, ylim = len(P[0]), len(P)
    beams = []
    for row in range(ylim):
        # left to right
        beams.append(Beam(x=0, y=row, xd=1, yd=0))
        # right to left
        beams.append(Beam(x=xlim-1, y=row, xd=-1, yd=0))

    for col in range(xlim):
        # top to bottom
        beams.append(Beam(x=col, y=0, xd=0, yd=1))
        # bottom to top
        beams.append(Beam(x=col, y=xlim-1, xd=0, yd=-1))

    return max(energise(b, P) for b in beams)

# part2
timer(reset=True)

maxenergy = energise_every_beam(P)
timer(f"Part 2: {maxenergy}")
pyperclip.copy(int(maxenergy))


