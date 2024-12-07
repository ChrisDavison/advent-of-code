import datetime
import functools
import heapq
import operator
import re
from collections import Counter, abc, defaultdict
from itertools import chain, combinations, islice
from math import gcd, inf
from pathlib import Path
from time import time_ns
from typing import Callable, Iterable, List, Optional, Sequence, Set, Tuple, Union

import pyperclip

T_START = None
T_NOW = None
lines = str.splitlines  # By default, split input text into lines


def paragraphs(text):
    "Split text into paragraphs"
    return text.split("\n\n")


def parse(
    day_or_text: Union[int, str],
    parser: Callable = str,
    section_by: Callable = lines,
    show=8,
) -> tuple:
    """Split the input text into `sections`, and apply `parser` to each.
    The first argument is either the text itself, or the day number of a text file."""
    text = get_text(day_or_text)
    show_items("Puzzle input", text.splitlines(), show)
    records = mapt(parser, section_by(text.rstrip()))
    if parser is not str or section_by != lines:
        show_items("Parsed representation", records, show)
    return records


def get_text(day_or_text: Union[int, str]) -> str:
    """The text used as input to the puzzle: either a string or the day number,
    which denotes the file 'AOC/year/input{day}.txt'."""
    if isinstance(day_or_text, str):
        return day_or_text
    else:
        filename = f"input/{day_or_text:02d}"
        return Path(filename).read_text()


def show_items(source, items, show: int, hr="─" * 100):
    """Show the first few items, in a pretty format."""
    if show:
        types = Counter(map(type, items))
        counts = ", ".join(
            f'{n} {t.__name__}{"" if n == 1 else "s"}' for t, n in types.items()
        )
        print(f"{hr}\n{source} ➜ {counts}:\n{hr}")
        for line in items[:show]:
            print(truncate(line))
        if show < len(items):
            print("...")


# =================================


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


def countwhere(pred, iterable) -> int:
    return sum(1 for item in iterable if pred(item))


flatten = chain.from_iterable  # Yield items from each sequence in turn


# =========================
# POINTS IN SPACE
# =========================
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


class Point2D:
    def __init__(self, x, y, data=None):
        self.x = x
        self.y = y
        if data:
            self.data = data

    def up(self):
        return Point2D(self.x, self.y - 1)

    def right(self):
        return Point2D(self.x + 1, self.y)

    def down(self):
        return Point2D(self.x, self.y + 1)

    def left(self):
        return Point2D(self.x - 1, self.y)

    def surrounding(self, directions=4):
        if directions == 4:
            yield self.up()
            yield self.right()
            yield self.down()
            yield self.left()
        if directions == 8:
            yield self.up().right()
            yield self.right().down()
            yield self.down().left()
            yield self.left().up()

    def __str__(self):
        return f"({self.x},{self.y})"

    def __repr__(self):
        return self.__str__()

    def __eq__(self, o):
        return self.x == o.x and self.y == o.y

    def __hash__(self):
        return (self.x, self.y).__hash__()


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


# ==============
# GRID
# ==============


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
        self.n_rows = len(grid)
        self.n_cols = len(grid[0])
        self.dim = (self.n_rows, self.n_cols)

    def __missing__(self, point):
        """If asked for a point off the grid, either return default or raise error."""
        if self.default is KeyError:
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
            if add2(point, Δ) in self or self.default is not KeyError
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
        import matplotlib.pyplot as plt

        plt.figure(figsize=figsize)
        plt.gca().invert_yaxis()
        for m in markers:
            plt.plot(*T(p for p in self if self[p] == m), markers[m], **kwds)


def neighbors(point, directions=directions4) -> List[Point]:
    """Neighbors of this point, in the given directions.
    (This function can be used outside of a Grid class.)"""
    return [add(point, Δ) for Δ in directions]


# =======
# UTILITY
# =======
class multimap(defaultdict):
    """A mapping of {key: [val1, val2, ...]}."""

    def __init__(self, pairs: Iterable[tuple] = (), symmetric=False):
        """Given (key, val) pairs, return {key: [val, ...], ...}.
        If `symmetric` is True, treat (key, val) as (key, val) plus (val, key)."""
        self.default_factory = list
        for key, val in pairs:
            self[key].append(val)
            if symmetric:
                self[val].append(key)


def prod(numbers) -> float:  # Will be math.prod in Python 3.8
    """The product formed by multiplying `numbers` together."""
    result = 1
    for x in numbers:
        result *= x
    return result


def T(matrix: Sequence[Sequence]) -> List[Tuple]:
    """The transpose of a matrix: T([(1,2,3), (4,5,6)]) == [(1,4), (2,5), (3,6)]"""
    return list(zip(*matrix))


def total(counter: Counter) -> int:
    """The sum of all the counts in a Counter."""
    return sum(counter.values())


def minmax(numbers) -> Tuple[int, int]:
    """A tuple of the (minimum, maximum) of numbers."""
    numbers = list(numbers)
    return min(numbers), max(numbers)


def cover(*integers) -> range:
    """A `range` that covers all the given integers, and any in between them.
    cover(lo, hi) is an inclusive (or closed) range, equal to range(lo, hi + 1).
    The same range results from cover(hi, lo) or cover([hi, lo])."""
    if len(integers) == 1:
        integers = the(integers)
    return range(min(integers), max(integers) + 1)


def the(sequence) -> object:
    """Return the one item in a sequence. Raise error if not exactly one."""
    for i, item in enumerate(sequence, 1):
        if i > 1:
            raise ValueError("Expected exactly one item in the sequence.")
    return item


def split_at(sequence, i) -> Tuple[Sequence, Sequence]:
    """The sequence split into two pieces: (before position i, and i-and-after)."""
    return sequence[:i], sequence[i:]


def ignore(*args) -> None:
    "Just return None."
    return None


def is_int(x) -> bool:
    "Is x an int?"
    return isinstance(x, int)


def sign(x) -> int:
    "0, +1, or -1"
    return 0 if x == 0 else +1 if x > 0 else -1


def lcm(i, j) -> int:
    "Least common multiple"
    return i * j // gcd(i, j)


def lcm_list(ll) -> int:
    "Least common multiple of a list of numbers"
    elem = ll[0]
    for i in ll[1:]:
        elem = lcm(elem, i)
    return elem


def union(sets) -> set:
    "Union of several sets"
    return set().union(*sets)


def intersection(sets):
    "Intersection of several sets; error if no sets."
    first, *rest = sets
    return set(first).intersection(*rest)


def naked_plot(points, marker="o", size=(10, 10), invert=True, square=False, **kwds):
    """Plot `points` without any axis lines or tick marks.
    Optionally specify size, whether square or not, and whether to invery y axis."""
    import matplotlib.pyplot as plt

    if size:
        plt.figure(figsize=((size, size) if is_int(size) else size))
    plt.plot(*T(points), marker, **kwds)
    if square:
        plt.axis("square")
    plt.axis("off")
    if invert:
        plt.gca().invert_yaxis()


def clock_mod(i, m) -> int:
    """i % m, but replace a result of 0 with m"""
    # This is like a clock, where 24 mod 12 is 12, not 0.
    return (i % m) or m


def invert_dict(dic) -> dict:
    """Invert a dict, e.g. {1: 'a', 2: 'b'} -> {'a': 1, 'b': 2}."""
    return {dic[x]: x for x in dic}


def walrus(name, value):
    """If you're not in 3.8, and you can't do `x := val`,
    then you can use `walrus('x', val)`."""
    globals()[name] = value
    return value


cat = "".join
cache = functools.lru_cache(None)
Ø = frozenset()  # empty set


def quantify(iterable, pred=bool) -> int:
    """Count the number of items in iterable for which pred is true."""
    return sum(1 for item in iterable if pred(item))


def dotproduct(vec1, vec2):
    """The dot product of two vectors."""
    return sum(map(operator.mul, vec1, vec2))


def powerset(iterable) -> Iterable[tuple]:
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return flatten(combinations(s, r) for r in range(len(s) + 1))


flatten = chain.from_iterable  # Yield items from each sequence in turn


def append(sequences) -> Sequence:
    "Append into a list"
    return list(flatten(sequences))


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


# ===============
# Data structures
# ===============
class PriorityQueue:
    """A queue in which the item with minimum key(item) is always popped first."""

    def __init__(self, items=(), key=lambda x: x):
        self.key = key
        self.items = []  # a heap of (score, item) pairs
        for item in items:
            self.add(item)

    def add(self, item):
        """Add item to the queue."""
        pair = (self.key(item), item)
        heapq.heappush(self.items, pair)

    def pop(self):
        """Pop and return the item with min f(item) value."""
        return heapq.heappop(self.items)[1]

    def top(self):
        return self.items[0][1]

    def __len__(self):
        return len(self.items)


class Hdict(dict):
    """A dict, but it is hashable."""

    def __hash__(self):
        return hash(tuple(sorted(self.items())))


class HCounter(Counter):
    """A Counter, but it is hashable."""

    def __hash__(self):
        return hash(tuple(sorted(self.items())))


class Graph(dict):
    """A graph of {node: [neighboring_nodes...]}.
    Can store other kwd attributes on it (which you can't do with a dict)."""

    def __init__(self, contents, **kwds):
        self.update(contents)
        self.__dict__.update(**kwds)


class AttrCounter(Counter):
    """A Counter, but `ctr['name']` and `ctr.name` are the same."""

    def __getattr__(self, attr):
        return self[attr]

    def __setattr__(self, attr, value):
        self[attr] = value


def timed(func):
    def inner(*args, **kwargs):
        start = time_ns()
        ret = func(*args, **kwargs)
        ns_delta = time_ns() - start
        delta = ns_delta / 1e6
        print(f"{func.__name__} ({delta:.0f}ms) = {ret}")
        if ret:
            return ret, ns_delta

    return inner


def surrounding(point, directions=directions8):
    point = tuple(point)
    for direction in directions:
        yield point[0] + direction[0], point[1] + direction[1]


def char_indices(line, ch, reverse=False):
    if not reverse:
        for i, ltr in enumerate(line):
            if ltr == ch:
                yield i
    else:
        for i, ltr in enumerate(line[::-1]):
            if ltr == ch:
                yield len(line) - i - 1


def enumerated_grid(data):
    for y, line in enumerate(data):
        for x, ch in enumerate(line):
            yield (x, y), ch


def timer(msg=None, reset=False):
    global T_START, T_NOW
    if reset:
        T_NOW = time_ns()
        return
    tstr = datetime.datetime.now().strftime("%T")
    if T_START is None:
        T_START = time_ns()
        T_NOW = time_ns()
        print(f"START {tstr}")
    else:
        now = time_ns()
        delta = now - T_NOW
        msg = tstr if not msg else msg
        print(f"Δ {delta/1e6:.0f}ms - {msg}")
        T_NOW = now


def clip(val):
    input("Enter to copy to clipboard.")
    pyperclip.copy(val)
    timer(reset=True)


def as_grid(paragraph):
    return [[ch for ch in line] for line in paragraph.splitlines()]


def un_grid(grid):
    return "\n".join("".join(row) for row in grid)


def get_dim(data):
    g = as_grid(data)
    return (len(g[0]), len(g))


def A_star_search(problem, h=None):
    """Search nodes with minimum f(n) = path_cost(n) + h(n) value first."""
    h = h or problem.h
    return best_first_search(problem, f=lambda n: n.path_cost + h(n))


def best_first_search(problem, f) -> "Node":
    "Search nodes with minimum f(node) value first."
    node = Node(problem.initial)
    frontier = PriorityQueue([node], key=f)
    reached = {problem.initial: node}
    while frontier:
        node = frontier.pop()
        if problem.is_goal(node.state):
            return node
        for child in expand(problem, node):
            s = child.state
            if s not in reached or child.path_cost < reached[s].path_cost:
                reached[s] = child
                frontier.add(child)
    return search_failure


class SearchProblem:
    """The abstract class for a search problem. A new domain subclasses this,
    overriding `actions` and perhaps other methods.
    The default heuristic is 0 and the default action cost is 1 for all states.
    When you create an instance of a subclass, specify `initial`, and `goal` states
    (or give an `is_goal` method) and perhaps other keyword args for the subclass."""

    def __init__(self, initial=None, goal=None, **kwds):
        self.__dict__.update(initial=initial, goal=goal, **kwds)

    def __str__(self):
        return "{}({!r}, {!r})".format(type(self).__name__, self.initial, self.goal)

    def actions(self, state):
        raise NotImplementedError

    def result(self, state, action):
        return action  # Simplest case: action is result state

    def is_goal(self, state):
        return state == self.goal

    def action_cost(self, s, a, s1):
        return 1

    def h(self, node):
        return 0  # Never overestimate!


class GridProblem(SearchProblem):
    """Problem for searching a grid from a start to a goal location.
    A state is just an (x, y) location in the grid."""

    def actions(self, loc):
        return self.grid.neighbors(loc)

    def result(self, loc1, loc2):
        return loc2

    def h(self, node):
        return taxi_distance(node.state, self.goal)


class Node:
    "A Node in a search tree."

    def __init__(self, state, parent=None, action=None, path_cost=0):
        self.__dict__.update(
            state=state, parent=parent, action=action, path_cost=path_cost
        )

    def __repr__(self):
        return f"Node({self.state}, path_cost={self.path_cost})"

    def __len__(self):
        return 0 if self.parent is None else (1 + len(self.parent))

    def __lt__(self, other):
        return self.path_cost < other.path_cost


search_failure = Node(
    "failure", path_cost=inf
)  # Indicates an algorithm couldn't find a solution.


def expand(problem, node):
    "Expand a node, generating the children nodes."
    s = node.state
    for action in problem.actions(s):
        s2 = problem.result(s, action)
        cost = node.path_cost + problem.action_cost(s, action, s2)
        yield Node(s2, node, action, cost)


def path_actions(node):
    "The sequence of actions to get to this node."
    if node.parent is None:
        return []
    return path_actions(node.parent) + [node.action]


def path_states(node):
    "The sequence of states to get to this node."
    if node in (search_failure, None):
        return []
    return path_states(node.parent) + [node.state]
