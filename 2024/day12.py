import utility as u
from pathlib import Path
from argparse import ArgumentParser


if __name__ == "__main__":
    parser = ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("-s", "--sample", action="store_true")
    mode.add_argument("-d", "--data", action="store_true")
    parser.add_argument("-n", default=25, type=int)
    parser.add_argument("--debug", action="store_true", default=False)
    parser.add_argument("input", nargs="*")
    args = parser.parse_args()

    DAYNUM = u.ints(Path(__file__).stem)[0]
    if args.sample:
        data = Path(f"input/{DAYNUM:02d}s").read_text().strip()
    elif args.data:
        data = Path(f"input/{DAYNUM:02d}").read_text().strip()
    elif args.input:
        data = " ".join(args.input)
    else:
        data = input("input: ")

    # ----------------------------------------
    #                SOLUTION
    # ----------------------------------------
    print(data)
