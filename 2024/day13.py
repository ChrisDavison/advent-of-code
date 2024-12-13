import utility as u
from pathlib import Path
from argparse import ArgumentParser
import numpy as np


def cost(a, b, prize, na, nb):
    lhs = na * a + nb * b
    if not np.all(lhs == prize):
        return None
    return (na * 3,)


def solve(a, b, prize, prices=np.array([3, 1]), tokens=100):
    minprice = None
    for na in range(tokens):
        for nb in range(tokens):
            lhs = na * a + nb * b
            # print(f"{lhs}")
            if not np.all(lhs == prize):
                continue
            price = na * prices[0] + nb * prices[1]
            if minprice:
                minprice = min(price, minprice)
            else:
                minprice = price
    return minprice or 0


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument("-d", "--debug", action="store_true", default=False)
    parser.add_argument("file", type=Path, nargs=1)
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    data = u.paragraphs(args.file[0].read_text().strip())

    parsed = [
        u.mapl(lambda x: np.array(x, dtype=int), u.mapl(u.ints, chunk.splitlines()))
        for chunk in data
    ]

    # ---------------------------------------- Solution
    min_tokens = 0
    for p in parsed:
        n_tokens = solve(*p)
        if n_tokens:
            min_tokens += n_tokens
    print(min_tokens)
