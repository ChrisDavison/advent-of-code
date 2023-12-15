from utility import *

SAMPLE = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""

DATA = Path("input/15").read_text()

def puzzlehash(s):
    h = 0
    for ch in s:
        o = ord(ch)
        h += o
        h *= 17
        h %= 256
    return h

timer()
print(f"{puzzlehash('rn=1') = :}")
print(f"{puzzlehash('HASH') = :}")

P = mapt(puzzlehash, DATA.replace('\n', '').split(','))
timer(f"Part 1: {sum(P)}")
pyperclip.copy(sum(P))
print(f"{sum(P) = :}")

