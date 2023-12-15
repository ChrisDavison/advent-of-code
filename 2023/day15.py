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

# print(f"{puzzlehash('rn=1') = :}")
# print(f"{puzzlehash('HASH') = :}")

# Part 1
P = mapt(puzzlehash, DATA.replace('\n', '').split(','))
timer(f"Part 1: {sum(P)}")
pyperclip.copy(sum(P))
print()


# Part 2
timer(reset=True)
P = [(part, puzzlehash(part)) for part in DATA.replace('\n', '').split(',')]
boxes = []
for _ in range(256):
    boxes.append(dict())
# print(f"{boxes = :}")
for part, h in P:
    m = re.search(r'(\w+)([=-])(\d+)*', part)
    label, *action = m.groups()
    # print(part, label, action, h)
    match action:
        case ('=', n):
            box = puzzlehash(label)
            # print(f"Update box {box} to include label {label} with lens {n}")
            boxes[box][label] = n
        case ('-', _):
            # print(f"Remove label {label} from box {h}")
            box = puzzlehash(label)
            if label in boxes[box]:
                del boxes[box][label]

s = 0
for i, box in enumerate(boxes, start=1):
    # print(box)
    if len(box) == 0:
        continue
    # print(i-1)
    for j, (k, v) in enumerate(box.items(), start=1):
        sb = i * j * int(v)
        # print(f' {j} - {k}: {v} {sb}')
        s += sb


timer(f"Part 2: {s}")
pyperclip.copy(s)
print()

