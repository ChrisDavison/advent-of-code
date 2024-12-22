from utility import *
import itertools as it


def secretgen(s):
    while True:
        s = secret(s)
        yield s


def secret(s):
    magic = 16777216 - 1
    s = (s ^ (s << 6)) & magic
    s = (s ^ (s >> 5)) & magic
    s = (s ^ (s << 11)) & magic
    return s


@cache
def ones_digit(n):
    return n % 10


def solve_part1(filename, limit=2000):
    data = parse(filename, int, show=0)
    tot = 0
    for n in data:
        for s in it.islice(secretgen(n), limit):
            tot += s
    print(f"{tot=}")


def seq_prices(secrets, win_length):
    prices = dict()
    ones = mapt(ones_digit, secrets)
    deltas = mapt(lambda xy: xy[1] - xy[0], zip(ones, ones[1:]))
    for win in sliding_window(list(zip(ones[1:], deltas)), win_length):
        price = Xs(win)[-1]
        seq = Ys(win)
        if seq not in prices:
            prices[seq] = price
    return prices


def solve_part2(filename, limit=2000):
    data = parse(filename, int, show=0)
    i = 0
    prices = defaultdict(lambda: [0 for _ in range(len(data))])
    for i, n in enumerate(data):
        secrets = [n] + list(it.islice(secretgen(n), limit))
        n_prices = seq_prices(secrets, win_length=4)
        for seq, price in n_prices.items():
            prices[seq][i] = price
    m = 0
    for seq, pp in prices.items():
        m = max(m, sum(pp))
    print(f"{m}")


if __name__ == "__main__":
    # solve_part1("22s", 10)
    # solve_part1("22")
    solve_part2("22", 2000)
