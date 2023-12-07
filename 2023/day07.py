import math as m
from pathlib import Path

from pandas.core.indexes.base import pprint_thing
from utility import *

SAMPLE = """32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"""

DATA = DAY(7)

Hand = namedtuple('Hand', 'setrank cardrank cards bid')

def cardhand1(s):
    cards, bid = s.split(" ")
    bid = int(bid)
    cardset = sorted(Counter(cards).values())[::-1]
    rank = "123456789TJQKA"
    cardranks = mapt(lambda x: rank.index(x), cards)
    match cardset:
        case [5]:
            return Hand(6, cardranks, cards, bid)
        case [4, 1]:
            return Hand(5, cardranks, cards, bid)
        case [3, 2]:
            return Hand(4, cardranks, cards, bid)
        case [3, 1, 1]:
            return Hand(3, cardranks, cards, bid)
        case [2, 2, 1]:
            return Hand(2, cardranks, cards, bid)
        case [2, 1, 1, 1]:
            return Hand(1, cardranks, cards, bid)
        case _:
            return Hand(0, cardranks, cards, bid)


def cardhand2(s):
    cards, bid = s.split(" ")
    bid = int(bid)
    nj, cards_noj = sum(1 for c in cards if c == 'J'), ''.join(c for c in cards if c != 'J')
    cardset = sorted(Counter(cards_noj).values())[::-1]
    rank2 = "J123456789TQKA"
    cardranks2 = mapt(lambda x: rank2.index(x), cards)
    print(f"{cards}\t{cardset = :} {nj = :}")
    match (cardset, nj):
        case ([5], x):
            return Hand(6, cardranks2, cards, bid)
        case ([4, 1], x):
            return Hand(5, cardranks2, cards, bid)
        case ([3, 2], x):
            return Hand(4, cardranks2, cards, bid)
        case ([3, 1, 1], x):
            return Hand(3, cardranks2, cards, bid)
        case ([2, 2, 1], x):
            return Hand(2, cardranks2, cards, bid)
        case ([2, 1, 1, 1], x):
            return Hand(1, cardranks2, cards, bid)
        case _:
            return Hand(0, cardranks2, cards, bid)

def p(data, cardgen):
    cards = mapl(cardgen, data.splitlines())
    return sorted(cards)

# --- part 1
def f(data):
    s = 0
    for i, hand in enumerate(p(data, cardhand1)):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s


print(f"1. {f(SAMPLE) = :}")
print(f"1. {f(DATA) = :}")


# --- part 2
def f2(data):
    s = 0
    for i, hand in enumerate(p(data, cardhand2)):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s


print(f"2. {f2(SAMPLE) = :}")
# print(f"2. {f2(DATA) = :}")
