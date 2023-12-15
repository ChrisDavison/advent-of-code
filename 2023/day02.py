from utility import Path, np, re, mapl, timed, lines, parse, timer

TEST_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

game_rx = re.compile(r"Game\s+(?P<id>\d+):\s+(?P<sets>.*)")

DATA = Path("input/02").read_text()


def parse_set(s):
    rgb = [0, 0, 0]
    rgbstr = "rgb"
    for d, w in re.findall(r"(\d+) (red|green|blue)+", s):
        rgb[rgbstr.index(w[0])] = int(d)
    return np.array(rgb)


def parse_game(line):
    if m := game_rx.search(line):
        return int(m.group("id")), np.max(
            mapl(np.array, map(parse_set, m.group("sets").split(";"))), axis=0
        )


timer()
lim = np.array([12, 13, 14])
wins = 0
p2 = 0
for gameid, gameset_max in parse(2, parse_game, lines, show=0):
    if np.all(gameset_max <= lim):
        wins += gameid
    p2 += np.prod(gameset_max)
timer(f"Part 1 + 2: {wins}, {p2}")

