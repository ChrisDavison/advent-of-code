import utility as u
from pathlib import Path
from argparse import ArgumentParser
import numpy as np
import tqdm


def solve(a, b, prize, part2=False):
    minprice = None
    n_tokens = 100
    prize += 10000000000000 if part2 else 0
    maxa = n_tokens if not part2 else int(np.ceil(np.max(prize / a)))
    print(f"a{a} b{b} p{prize} %{prize % a} /{prize/a}")
    for na in range(maxa):
        aa = max(na, 1) * a
        pz = prize - aa
        rem = pz / b
        if rem[0] != rem[1] or rem[1]:
            continue
        price = 3 * na + rem[1]
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
    for i, p in tqdm.tqdm(enumerate(parsed)):
        min_tokens += solve(*p)
    print(min_tokens)

    min_tokens2 = 0
    for i, p in enumerate(parsed):
        min_tokens2 += solve(*p, part2=True)
    print(min_tokens2)
