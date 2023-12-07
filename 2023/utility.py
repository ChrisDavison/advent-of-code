from collections import namedtuple, Counter, defaultdict
from dataclasses import dataclass
from itertools import *
from itertools import chain
from pathlib import Path
from typing import List, Tuple, Union, Sequence, Iterable, Optional, Set
import functools
import math as m
import os
import pickle
import re
import operator

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import networkx as nx
from datetime import time



DAY = lambda day: Path(f"input/day{day:02d}").read_text()


class ErrorTodo(Exception):
    # Error representing something not yet implemented
    pass


def mapl(func, iterable) -> List:
    return list(map(func, iterable))


def mapt(func, iterable) -> Tuple:
    return tuple(map(func, iterable))


def truncate(object, width=100, ellipsis=" ...") -> str:
    """Use elipsis to truncate `str(object)` to `width` characters, if necessary."""
    string = str(object)
    return (
        string if len(string) <= width else string[: width - len(ellipsis)] + ellipsis
    )


# === Splitting regions of text ===

lines = str.splitlines


def paragraphs(text):
    return text.split("\n\n")


# === Parsing parts of text ===

Char = str  # Intended as the type of a one-character string
Atom = Union[str, float, int]  # The type of a string or number


def ints(text: str) -> Tuple[int]:
    """A tuple of all the integers in text, ignoring non-number characters."""
    return mapt(int, re.findall(r"-?[0-9]+", text))


def positive_ints(text: str) -> Tuple[int]:
    """A tuple of all the integers in text, ignoring non-number characters."""
    return mapt(int, re.findall(r"[0-9]+", text))


def digits(text: str) -> Tuple[int]:
    """A tuple of all the digits in text (as ints 0–9), ignoring non-digit characters."""
    return mapt(int, re.findall(r"[0-9]", text))


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
    words = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ]
    match s:
        case w if w in words:
            return words.index(w)
        case d if str.isdigit(w):
            return int(d)
        case _:
            return 0


def words(text: str) -> Tuple[str]:
    """A tuple of all the alphabetic words in text, ignoring non-letters."""
    return tuple(re.findall(r"[a-zA-Z]+", text))


def atoms(text: str) -> Tuple[Atom]:
    """A tuple of all the atoms (numbers or identifiers) in text. Skip punctuation."""
    return mapt(atom, re.findall(r"[+-]?\d+\.?\d*|\w+", text))


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


def is_int(x) -> bool:
    "Is x an int?"
    return isinstance(x, int)


def sign(x) -> int:
    "0, +1, or -1"
    return 0 if x == 0 else +1 if x > 0 else -1


def lcm(i, j) -> int:
    "Least common multiple"
    return i * j // gcd(i, j)


def union(sets) -> set:
    "Union of several sets"
    return set().union(*sets)


def intersection(sets):
    "Intersection of several sets; error if no sets."
    first, *rest = sets
    return set(first).intersection(*rest)


def clock_mod(i, m) -> int:
    """i % m, but replace a result of 0 with m"""
    # This is like a clock, where 24 mod 12 is 12, not 0.
    return (i % m) or m


cat = "".join
cache = functools.lru_cache(None)
Ø = frozenset()  # empty set


def countwhere(pred, iterable) -> int:
    return sum(1 for item in iterable if pred(item))


def powerset(iterable) -> Iterable[tuple]:
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return flatten(combinations(s, r) for r in range(len(s) + 1))


flatten = chain.from_iterable  # Yield items from each sequence in turn


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
    return (sequence[i : i + n] for i in range(len(sequence) + 1 - n))


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


Point = Tuple[int, ...]  # Type for points
Vector = Point  # E.g., (1, 0) can be a point, or can be a direction, a Vector
Zero = (0, 0)

directions4 = East, South, West, North = ((1, 0), (0, 1), (-1, 0), (0, -1))
diagonals = SE, NE, SW, NW = ((1, 1), (1, -1), (-1, 1), (-1, -1))
directions8 = directions4 + diagonals
directions5 = directions4 + (Zero,)
directions9 = directions8 + (Zero,)
arrow_direction = {
    "^": North,
    "v": South,
    ">": East,
    "<": West,
    ".": Zero,
    "U": North,
    "D": South,
    "R": East,
    "L": West,
}


def X_(point) -> int:
    "X coordinate of a point"
    return point[0]


def Y_(point) -> int:
    "Y coordinate of a point"
    return point[1]


def Z_(point) -> int:
    "Z coordinate of a point"
    return point[2]


def Xs(points) -> Tuple[int]:
    "X coordinates of a collection of points"
    return mapt(X_, points)


def Ys(points) -> Tuple[int]:
    "Y coordinates of a collection of points"
    return mapt(Y_, points)


def Zs(points) -> Tuple[int]:
    "X coordinates of a collection of points"
    return mapt(Z_, points)


def add(p: Point, q: Point) -> Point:
    return mapt(operator.add, p, q)


def sub(p: Point, q: Point) -> Point:
    return mapt(operator.sub, p, q)


def neg(p: Point) -> Vector:
    return mapt(operator.neg, p)


def mul(p: Point, k: float) -> Vector:
    return tuple(k * c for c in p)


def distance(p: Point, q: Point) -> float:
    """Euclidean (L2) distance between two points."""
    d = sum((pi - qi) ** 2 for pi, qi in zip(p, q)) ** 0.5
    return int(d) if d.is_integer() else d


def slide(points: Set[Point], delta: Vector) -> Set[Point]:
    """Slide all the points in the set of points by the amount delta."""
    return {add(p, delta) for p in points}


def make_turn(facing: Vector, turn: str) -> Vector:
    """Turn 90 degrees left or right. `turn` can be 'L' or 'Left' or 'R' or 'Right' or lowercase."""
    (x, y) = facing
    return (y, -x) if turn[0] in ("L", "l") else (-y, x)


# Profiling found that `add` and `taxi_distance` were speed bottlenecks;
# I define below versions that are specialized for 2D points only.


def add2(p: Point, q: Point) -> Point:
    """Specialized version of point addition for 2D Points only. Faster."""
    return (p[0] + q[0], p[1] + q[1])


def taxi_distance(p: Point, q: Point) -> int:
    """Manhattan (L1) distance between two 2D Points."""
    return abs(p[0] - q[0]) + abs(p[1] - q[1])


class Grid(dict):
    """A 2D grid, implemented as a mapping of {(x, y): cell_contents}."""

    def __init__(self, grid=(), directions=directions4, skip=(), default=KeyError):
        """Initialize with either (e.g.) `Grid({(0, 0): '#', (1, 0): '.', ...})`, or
        `Grid(["#..", "..#"]) or `Grid("#..\n..#")`."""
        self.directions = directions
        self.default = default
        if isinstance(grid, abc.Mapping):
            self.update(grid)
        else:
            if isinstance(grid, str):
                grid = grid.splitlines()
            self.update(
                {
                    (x, y): val
                    for y, row in enumerate(grid)
                    for x, val in enumerate(row)
                    if val not in skip
                }
            )

    def __missing__(self, point):
        """If asked for a point off the grid, either return default or raise error."""
        if self.default == KeyError:
            raise KeyError(point)
        else:
            return self.default

    def copy(self):
        return Grid(self, directions=self.directions, default=self.default)

    def neighbors(self, point) -> List[Point]:
        """Points on the grid that neighbor `point`."""
        return [
            add2(point, Δ)
            for Δ in self.directions
            if add2(point, Δ) in self or self.default != KeyError
        ]

    def neighbor_contents(self, point) -> Iterable:
        """The contents of the neighboring points."""
        return (self[p] for p in self.neighbors(point))

    def to_rows(self, xrange=None, yrange=None) -> List[List[object]]:
        """The contents of the grid, as a rectangular list of lists.
        You can define a window with an xrange and yrange; or they default to the whole grid.
        """
        xrange = xrange or cover(Xs(self))
        yrange = yrange or cover(Ys(self))
        default = " " if self.default is KeyError else self.default
        return [[self.get((x, y), default) for x in xrange] for y in yrange]

    def print(self, sep="", xrange=None, yrange=None):
        """Print a representation of the grid."""
        for row in self.to_rows(xrange, yrange):
            print(*row, sep=sep)

    def plot(self, markers={"#": "s", ".": ","}, figsize=(14, 14), **kwds):
        """Plot a representation of the grid."""
        plt.figure(figsize=figsize)
        plt.gca().invert_yaxis()
        for m in markers:
            plt.plot(*T(p for p in self if self[p] == m), markers[m], **kwds)


def neighbors(point, directions=directions4) -> List[Point]:
    """Neighbors of this point, in the given directions.
    (This function can be used outside of a Grid class.)"""
    return [add(point, Δ) for Δ in directions]
