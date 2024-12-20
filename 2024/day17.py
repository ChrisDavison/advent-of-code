from utility import paragraphs, ints, parse, atoms

DEBUG = False


class Program:
    def __init__(self, a, b, c, instructions):
        self.instructions = instructions
        self.a = a
        self.b = b
        self.c = c
        self.pointer = 0
        self.output = []

    @staticmethod
    def from_file(filename):
        parsed = parse(filename, atoms, show=0)
        a = b = c = 0
        instructions = []
        for line in parsed:
            match line:
                case ["Register", "A", val]:
                    a = val
                case ["Register", "B", val]:
                    b = val
                case ["Register", "C", val]:
                    c = val
                case ["Program", *vals]:
                    instructions = vals
        return Program(a, b, c, instructions)

    def __str__(self):
        outs = []
        outs.append("    " + " " * (2 * self.pointer) + "v")
        outs.append("PRG " + ",".join(map(str, self.instructions)))
        register = f"a{self.a} b{self.b} c{self.c}"
        outs.append(f"VAL {self.instructions[self.pointer+1]} (CV {self.cv})")
        outs.append(f"REG {register}")
        outputstr = " ".join(map(str, self.output))
        outs.append(f"OUT {outputstr}")
        return "\n".join(outs)

    def adv(self):  # opcode 0
        # adv. div register['A'] by v
        # truncate to integer
        # store in A
        if DEBUG:
            print(f"OP: A / 2**cv -> {self.a} / {2**self.cv} = {self.a /2**self.cv}")
        self.a = int(self.a / (2**self.cv))

    def bxl(self):  # opcode 1
        # bitwise XOR of B and self.v
        # stored in B
        if DEBUG:
            print(f"OP: B XOR v -> {self.b} ^ {self.v} = {self.b ^ self.v}")
        self.b = self.b ^ self.v

    def bst(self):  # opcode 2
        # self.v % 8, stored into B
        if DEBUG:
            print(f"OP: cv % 8 -> {self.cv} % 8 = {self.cv % 8}")
        self.b = self.cv % 8

    def jnz(self):  # opcode 3
        # if A is 0, nothing
        # else jump to position self.v
        if DEBUG:
            print(f"OP: jnz A, jump to v -> {self.a}? {self.v}")
        if self.a != 0:
            self.pointer = self.v
            return True

    def bxc(self):  # opcode 4
        # bitwise XOR of B and C, stored in B
        # ignors self.v
        if DEBUG:
            print(f"OP: B = B ^ C -> {self.b} ^ {self.c} = {self.b^self.c}")
        self.b = self.b ^ self.c

    def out(self):  # opcode 5
        # self.v % 8, output self.v (append, or insert?)
        if DEBUG:
            print(f"OP: output cv % 8 -> {self.cv} % 8 = {self.cv % 8}")
        self.output.append(self.cv % 8)

    def bdv(self):  # opcode 6
        if DEBUG:
            print(
                f"OP: B = A / 2**cv -> {self.a} / {2**self.cv} = {self.a / 2**self.cv}"
            )
        self.b = int(self.a / (2**self.cv))

    def cdv(self):  # opcode 7
        if DEBUG:
            print(
                f"OP: C = A / 2**cv -> {self.a} / {2**self.cv} = {self.a / 2**self.cv}"
            )
        self.c = int(self.a / (2**self.cv))

    @property
    def v(self):
        return self.instructions[self.pointer + 1]

    @property
    def cv(self):
        match self.v:
            case 0 | 1 | 2 | 3 | 7:
                return self.v
            case 4:
                return self.a
            case 5:
                return self.b
            case 6:
                return self.c

    def execute(self, step=False, part2=False):
        while True:
            if self.pointer >= len(self.instructions):
                break
            self.value = self.instructions[self.pointer + 1]
            if step:
                print(self)

            jumped = False
            match self.instructions[self.pointer]:
                case 0:
                    self.adv()
                case 1:
                    self.bxl()
                case 2:
                    self.bst()
                case 3:
                    jumped = self.jnz()
                case 4:
                    self.bxc()
                case 5:
                    self.out()
                case 6:
                    self.bdv()
                case 7:
                    self.cdv()

            if not jumped:
                self.pointer += 2
            if step:
                input()
                print()
        return ",".join(map(str, self.output)), False


def part1(filename):
    p = Program.from_file(filename)
    print(f"part1 (file {filename})", p.execute()[0])


def part2sample():
    """The program is bespoke and needs decompiling

    while a != 0:
        a = a / 2**3
        out a
        break
    """
    a = b = c = 0
    out = []
    while a != 0:
        b = (a % 8) ** 2
        c = a >> b
        a = a >> a
        b ^= 7 ^ c
        out.append(b)
        break


def part2():
    """The program is bespoke and needs decompiling

    Mine breaks down to...
    b = a%7
    b = b^2
    c = a/2^b
    a = a/2^a
    b = b^7
    b = b^c
    out b
    if not a, break

    ...simplified
    b = (a%8)^2
    c = a >> b
    a = a >> a
    b ^= 7 ^ c
    out b
    if not a, break
    """
    a = b = c = 0
    out = []
    p = Program.from_file("17")
    instructions = list(p.instructions)
    print(f"{instructions=}")
    a = p.a
    while True:
        b = (a % 8) ** 2
        c = a >> b
        a = a >> a
        b ^= 7 ^ c
        out.append(b)
        if instructions[: len(out)] != out:
            print("INVALID A")
            a += 1
            b = c = 0
        else:
            required_bits = a % 8
        break
        if not a:
            break


part1("17s")
part1("17")
part2()
