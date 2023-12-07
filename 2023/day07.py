import math as m
from pathlib import Path

from pandas.core.indexes.base import pprint_thing
from utility import *
from time import process_time_ns, time_ns

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
    cardranks = mapt(lambda x: "123456789TJQKA".index(x), cards)
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
    cardset = sorted(Counter(cards_noj).values())[::-1] if cards_noj else [0]
    cardranks = mapt(lambda x: "J123456789TQKA".index(x), cards)
    cardset[0] += nj
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

start = time_ns()
print(f"1. {f(SAMPLE) = :}")
print(f"1. {f(DATA) = :}")
mid = time_ns()
print(f"\t{(mid - start) / 1e6:.0f}ms")

print("-----------")

# --- part 2
def f2(data):
    s = 0
    for i, hand in enumerate(p(data, cardhand2)):
        score = hand.bid * (i + 1)
        # print(f"{hand.cards} \t\t {hand.bid} * {i + 1} = {score}")
        s += score
    return s

print(f"2. {f2(SAMPLE) = :}")
print(f"2. {f2(DATA) = :}")
end = time_ns()
print(f"\t{(end - mid) / 1e6:.0f}ms")
print(f"Total {(end - start) / 1e6:.0f} ms")
