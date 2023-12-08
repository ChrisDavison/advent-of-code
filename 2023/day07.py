from utility import *

SAMPLE = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

DATA = Path("input/07").read_text()

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


@timed
def part1(data=SAMPLE):
    s = 0
    ordered_hands = sorted(
        [cardhand(line, j_is_wild=False) for line in data.splitlines()]
    )
    for i, hand in enumerate(ordered_hands):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s


@timed
def part2(data=SAMPLE):
    s = 0
    ordered_hands = sorted(
        [cardhand(line, j_is_wild=True) for line in data.splitlines()]
    )
    for i, hand in enumerate(ordered_hands):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s


part1(DATA)
part2(DATA)
