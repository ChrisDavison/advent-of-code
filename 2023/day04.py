from utility import *
from pprint import pprint as pp

SAMPLE = """Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"""

DATA = Path('input/04').read_text()


def parse_game(l):
    if m := re.search(r"Card\s+(\d+):\s+(.+)\s+\|\s(.+)", l):
        card = int(m[1])
        winning = set(mapt(int, m[2].split()))
        have = set(mapt(int, m[3].split()))
        return card, winning, have


p = parse(4, parse_game, lines, show=0)

s = 0
for _id, have, win in p:
    inter = have & win
    if inter:
        s += 2 ** (len(inter) - 1)
print(s)


copies_of_each = [1] * (len(p) + 1)
copies_of_each[0] = 0
s = 0

for gameid, have, win in p:
    inter = have & win
    if not inter:
        continue
    for g in range(1, len(inter) + 1):
        copies_of_each[gameid + g] += copies_of_each[gameid]

pp(sum(copies_of_each))
