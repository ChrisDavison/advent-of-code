from utility import Path, timer, re, cache
import pyperclip

SAMPLE = """rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"""

DATA = Path("input/15").read_text()

timer(reset=True)

@cache
def puzzlehash(s):
    h = 0
    for ch in s:
        h += ord(ch)
        h *= 17
        h %= 256
    return h
    # could also implement as reduce, but it's slightly slower
    # return reduce(lambda current, next: ((current + ord(next)) * 17) % 256, s, 0)


timer()

# Part 1
P = [puzzlehash(part) for part in DATA.replace('\n', '').split(',')]
timer(f"Part 1: {sum(P)}")
pyperclip.copy(sum(P))

# Part 2
timer(reset=True)
P = [(part, puzzlehash(part)) for part in DATA.replace('\n', '').split(',')]
boxes = []
for _ in range(256):
    boxes.append(dict())
for part, h in P:
    m = re.search(r'(\w+)([=-])(\d+)*', part)
    label, *action = m.groups()
    box = puzzlehash(label)
    match action:
        case ('=', focal_len):
            boxes[box][label] = focal_len
        case ('-', _):
            if label in boxes[box]:
                del boxes[box][label]
s = 0
for i, box in enumerate(boxes, start=1):
    if len(box) == 0:
        continue
    for j, (_, v) in enumerate(box.items(), start=1):
        s += i * j * int(v)
timer(f"Part 2: {s}")
pyperclip.copy(s)

