from utility import *
from time import  time_ns

SAMPLE = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

DATA = DAY(7)

Hand = namedtuple("Hand", "setrank cardrank cards bid")

def cardset_to_setrank(cs):
    ranks = [[1, 1, 1, 1, 1], [2, 1, 1, 1], [2, 2, 1], [3, 1, 1], [3, 2], [4, 1], [5]]
    return ranks.index(cs)

def cardhand(cardstr, j_is_wild=False):
    cards, bid = cardstr.split(" ")
    bid = int(bid)
    nj, cards_noj = sum(1 for c in cards if c == "J"), "".join(
        c for c in cards if c != "J"
    )
    cards_to_count = cards if not j_is_wild else cards_noj
    cardset = (
        sorted(Counter(cards_to_count).values(), reverse=True)
        if cards_to_count
        else [0]
    )
    if j_is_wild:
        cardset[0] += nj
    rank = "123456789TJQKA" if not j_is_wild else "J123456789TQKA"
    cardranks = mapt(lambda x: rank.index(x), cards)
    return Hand(cardset_to_setrank(cardset), cardranks, cards, bid)

def parse_cards(line):
    return cardhand(line, False)

def parse_cards_wild_j(line):
    return cardhand(line, True)

def score(data, parser):
    s = 0
    for i, hand in enumerate(parse(data, parser, lines, show=0)):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s


start = time_ns()
print(f"{score(SAMPLE, parse_cards) = :}")
print(f"{score(DATA, parse_cards) = :}")

mid = time_ns()
print(f"\t{(mid - start) / 1e6:.0f}ms")

print("-----------")

print(f"{score(SAMPLE, parse_cards_wild_j) = :}")
print(f"{score(DATA, parse_cards_wild_j) = :}")
end = time_ns()
print(f"\t{(end - mid) / 1e6:.0f}ms")
print(f"Total {(end - start) / 1e6:.0f} ms")
