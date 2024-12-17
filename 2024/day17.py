from utility import paragraphs, ints
from pathlib import Path
from collections import defaultdict

DAYNUM = ints(Path(__file__).stem)[0]
fsample = Path(f"input/{DAYNUM}s")
fdata = Path(f"input/{DAYNUM}")


def parse(filename):
    registerstr, program = paragraphs(filename.read_text())
    registers = defaultdict(int)
    for line in registerstr.splitlines():
        lhs, rhs = line.split(":", 1)
        registers[lhs.split(" ", 1)[1]] = int(rhs.strip())
    program = list(ints(program))
    return program, registers


class Program:
    def __init__(self, instructions, registers):
        self.instructions = instructions
        self.registers = registers
        self.pointer = 0
        self.output = []

    def __str__(self):
        outs = []
        outs.append("    " + " " * (2 * self.pointer) + "v")
        outs.append("PRG " + ",".join(map(str, self.instructions)))
        register = " ".join(k + str(v) for k, v in self.registers.items())
        outs.append(f"VAL {self.instructions[self.pointer+1]} (CV {self.cv})")
        outs.append(f"REG {register}")
        outputstr = " ".join(map(str, self.output))
        outs.append(f"OUT {outputstr}")
        return "\n".join(outs)

    def adv(self):  # opcode 0
        # adv. div register['A'] by v
        # truncate to integer
        # store in A
        print(f"OP: A / 2**cv -> {self.A} / {2**self.cv} = {self.A /2**self.cv}")
        self.A = int(self.A / (2**self.cv))

    def bxl(self):  # opcode 1
        # bitwise XOR of B and self.v
        # stored in B
        print(f"OP: B XOR v -> {self.B} ^ {self.v} = {self.B ^ self.v}")
        self.B = self.B ^ self.v

    def bst(self):  # opcode 2
        # self.v % 8, stored into B
        print(f"OP: cv % 8 -> {self.cv} % 8 = {self.cv % 8}")
        self.B = self.cv % 8

    def jnz(self):  # opcode 3
        # if A is 0, nothing
        # else jump to position self.v
        print(f"OP: jnz A, jump to v -> {self.A}? {self.v}")
        if self.A != 0:
            self.pointer = self.v
            return True

    def bxc(self):  # opcode 4
        # bitwise XOR of B and C, stored in B
        # ignors self.v
        print(f"OP: B = B ^ C -> {self.B} ^ {self.C} = {self.B^self.C}")
        self.B = self.B ^ self.C

    def out(self):  # opcode 5
        # self.v % 8, output self.v (append, or insert?)
        print(f"OP: output cv % 8 -> {self.cv} % 8 = {self.cv % 8}")
        self.output.append(self.cv % 8)

    def bdv(self):  # opcode 6
        print(f"OP: B = A / 2**cv -> {self.A} / {2**self.cv} = {self.A / 2**self.cv}")
        self.B = int(self.A / (2**self.cv))

    def cdv(self):  # opcode 7
        print(f"OP: C = A / 2**cv -> {self.A} / {2**self.cv} = {self.A / 2**self.cv}")
        self.C = int(self.A / (2**self.cv))

    @property
    def A(self):
        return self.registers["A"]

    @A.setter
    def A(self, value):
        self.registers["A"] = value

    @property
    def B(self):
        return self.registers["B"]

    @B.setter
    def B(self, value):
        self.registers["B"] = value

    @property
    def C(self):
        return self.registers["C"]

    @C.setter
    def C(self, value):
        self.registers["C"] = value

    @property
    def v(self):
        return self.instructions[self.pointer + 1]

    @property
    def cv(self):
        if self.v in [0, 1, 2, 3]:
            return self.v
        elif self.v == 4:
            return self.registers["A"]
        elif self.v == 5:
            return self.registers["B"]
        elif self.v == 6:
            return self.registers["C"]
        elif self.v == 7:
            return 7

    def execute(self, step=False):
        while True:
            if self.pointer >= len(self.instructions):
                break
            self.value = self.instructions[self.pointer + 1]
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
        return ",".join(map(str, self.output))


# execute([2, 6], {"C": 9})
