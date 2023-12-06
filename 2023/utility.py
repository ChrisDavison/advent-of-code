from collections import namedtuple, Counter, defaultdict
from dataclasses import dataclass
# from itertools import *
from itertools import chain
from pathlib import Path
from typing import List, Tuple, Union, Sequence, Iterable, Optional
import functools
import math as m
import os
import pickle
import re

# from matplotlib.backend_bases import MouseButton
# from skimage import transform, data
# from skimage.color import rgb2gray
# from skimage.draw import polygon2mask
# from skimage.feature import match_descriptors, plot_matches, SIFT
# from skimage.io import imread, imshow
# import cv2
# import matplotlib.pyplot as plt
# import numpy as np
# import pandas as pd


DAY = lambda day: Path(f'input/day{day:02d}').read_text()


class ErrorTodo(Exception):
    # Error representing something not yet implemented
    pass


def mapl(func, iterable) -> List:
    return list(map(func, iterable))

def mapt(func, iterable) -> Tuple:
    return tuple(map(func, iterable))

def truncate(object, width=100, ellipsis=' ...') -> str:
    """Use elipsis to truncate `str(object)` to `width` characters, if necessary."""
    string = str(object)
    return string if len(string) <= width else string[:width-len(ellipsis)] + ellipsis

# === Splitting regions of text ===

lines = str.splitlines

def paragraphs(text):
    return text.split("\n\n")

# === Parsing parts of text ===

Char = str # Intended as the type of a one-character string
Atom = Union[str, float, int] # The type of a string or number

def ints(text: str) -> Tuple[int]:
    """A tuple of all the integers in text, ignoring non-number characters."""
    return mapt(int, re.findall(r'-?[0-9]+', text))

def positive_ints(text: str) -> Tuple[int]:
    """A tuple of all the integers in text, ignoring non-number characters."""
    return mapt(int, re.findall(r'[0-9]+', text))

def digits(text: str) -> Tuple[int]:
    """A tuple of all the digits in text (as ints 0–9), ignoring non-digit characters."""
    return mapt(int, re.findall(r'[0-9]', text))


def digits_and_worddigits(text: str) -> Tuple[int]:
    """A tuple of all the digits in text (as ints 0–9), ignoring non-digit characters."""
    seen = set()
    rx = re.compile("(one|two|three|four|five|six|seven|eight|nine|[0-9])")
    nums = []
    for i in range(0, len(text)):
        if m := rx.search(text, i):
            r = (m.start(), m.end())
            if r not in seen:
                seen.add(r)
                nums.append(m.group(1))
    return mapt(word_to_num, nums)

def word_to_num(s):
    words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
    match s:
        case w if w in words:
            return words.index(w)
        case d if str.isdigit(w):
            return int(d)
        case _:
            return 0


def words(text: str) -> Tuple[str]:
    """A tuple of all the alphabetic words in text, ignoring non-letters."""
    return tuple(re.findall(r'[a-zA-Z]+', text))

def atoms(text: str) -> Tuple[Atom]:
    """A tuple of all the atoms (numbers or identifiers) in text. Skip punctuation."""
    return mapt(atom, re.findall(r'[+-]?\d+\.?\d*|\w+', text))

def atom(text: str) -> Atom:
    """Parse text into a single float or int or str."""
    try:
        x = float(text)
        return round(x) if x.is_integer() else x
    except ValueError:
        return text.strip()


def minmax(numbers) -> Tuple[int, int]:
    """A tuple of the (minimum, maximum) of numbers."""
    numbers = list(numbers)
    return min(numbers), max(numbers)

def T(matrix: Sequence[Sequence]) -> List[Tuple]:
    """The transpose of a matrix: T([(1,2,3), (4,5,6)]) == [(1,4), (2,5), (3,6)]"""
    return list(zip(*matrix))

def split_at(sequence, i) -> Tuple[Sequence, Sequence]:
    """The sequence split into two pieces: (before position i, and i-and-after)."""
    return sequence[:i], sequence[i:]

def is_int(x) -> bool: "Is x an int?"; return isinstance(x, int)

def sign(x) -> int: "0, +1, or -1"; return (0 if x == 0 else +1 if x > 0 else -1)

def lcm(i, j) -> int: "Least common multiple"; return i * j // gcd(i, j)

def union(sets) -> set: "Union of several sets"; return set().union(*sets)

def intersection(sets):
    "Intersection of several sets; error if no sets."
    first, *rest = sets
    return set(first).intersection(*rest)

def clock_mod(i, m) -> int:
    """i % m, but replace a result of 0 with m"""
    # This is like a clock, where 24 mod 12 is 12, not 0.
    return (i % m) or m

cat     = ''.join
cache   = functools.lru_cache(None)
Ø       = frozenset() # empty set

def countwhere(pred, iterable) -> int:
    return sum(1 for item in iterable if pred(item))

def powerset(iterable) -> Iterable[tuple]:
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return flatten(combinations(s, r) for r in range(len(s) + 1))

flatten = chain.from_iterable # Yield items from each sequence in turn

def batched(iterable, n) -> Iterable[tuple]:
    "Batch data into non-overlapping tuples of length n. The last batch may be shorter."
    # batched('ABCDEFG', 3) --> ABC DEF G
    it = iter(iterable)
    while True:
        batch = tuple(islice(it, n))
        if batch:
            yield batch
        else:
            return

def sliding_window(sequence, n) -> Iterable[Sequence]:
    """All length-n subsequences of sequence."""
    return (sequence[i:i+n] for i in range(len(sequence) + 1 - n))

def first(iterable, default=None) -> Optional[object]:
    """The first element in an iterable, or the default if iterable is empty."""
    return next(iter(iterable), default)

def last(iterable) -> Optional[object]:
    """The last element in an iterable."""
    for item in iterable:
        pass
    return item

def nth(iterable, n, default=None):
    "Returns the nth item or a default value"
    return next(islice(iterable, n, None), default)

def first_true(iterable, default=False):
    """Returns the first true value in the iterable.
    If no true value is found, returns `default`."""
    return next((x for x in iterable if x), default)
