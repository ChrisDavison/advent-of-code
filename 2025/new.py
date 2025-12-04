#!/usr/bin/env python3
from argparse import ArgumentParser
from datetime import date
import os
import requests

today = date.today()

parser = ArgumentParser()
parser.add_argument("--year", default=today.year, type=int)
parser.add_argument("--day", default=today.day, type=int)
parser.add_argument("-s", "--sample-from-clipboard", action="store_true", default=False)
args = parser.parse_args()

# ------------------------------------------------------------
#                           get data
# ------------------------------------------------------------
s = requests.Session()
s.cookies.set(
    "session",
    open(os.path.expanduser("~/.aoc_token")).read().strip(),
)
response = s.get(f"https://adventofcode.com/{args.year}/day/{args.day}/input")
print(response.text)

with open(f"input/{args.day:02d}", "w") as f:
    f.write(response.text)

# ------------------------------------------------------------
#                     save clipboard sample
# ------------------------------------------------------------
if args.sample_from_clipboard:
    import pyperclip

    sample = pyperclip.paste()
    with open(f"input/{args.day:02d}s", "w") as f:
        f.write(sample)

# ------------------------------------------------------------
#                        write template
# ------------------------------------------------------------
template = f"""from utility import *

ds = parse("{args.day:02d}s")

def part1(data):
    pass

def part2(data):
    pass

part1(ds)
"""
with open(f"day{args.day:02d}.py", "w") as f:
    f.write(template)
