DEBUG = False

def maybe_print(msg):
    DEBUG
    if DEBUG:
        print(msg)


class Machine:
    def __init__(self, opcodes):
        self.ops = opcodes
        self.ptr = 0
        self.halted = False
        self.mem = dict()
        self.modes = []
        
    def run(self):
        while not self.halted:
            self.one_step()
        return self.ops[0]

    def parse_param_modes(self, s):
        if not s:
            self.modes = [0, 0, 0]
            return self.modes
        s = int(s)
        a = int(s / 100)
        s = s % 10
        b = int(s / 10)
        s = s%10
        c = int(s)
        self.modes = [a, b, c]
        return self.modes
    
    def one_step(self):
        if self.halted or self.ptr >= len(self.ops):
            return
        opstr = str(self.ops[self.ptr])
        op = 0
        self.modes = [0, 0, 0]
        if len(opstr) > 1:
            op = int(opstr[-2:])
            self.parse_param_modes(opstr[:-2])
        else:
            op = int(opstr)

        if op == 1:
            self.op1()
        elif op == 2:
            self.op2()
        elif op == 3:
            self.op3()
        elif op == 4:
            self.op4()
        else:
            return self.halt()

    def op1(self):
        maybe_print(f"OP + (*{self.ptr}) {self.ops[self.ptr:self.ptr+3]}")
        a = self.ops[self.ptr + 1]
        if self.modes[0] == 0:
            a = self.ops[a]
        b = self.ops[self.ptr + 2]
        if self.modes[1] == 0:
            b = self.ops[b]
        c = self.ops[self.ptr + 3]
        maybe_print(f"a={a} b={b} c={c}")
        self.ops[c] = a + b
        self.ptr += 4

    def op2(self):
        maybe_print(f"OP * (*{self.ptr}) {self.ops[self.ptr:self.ptr+3]}")
        a = self.ops[self.ptr + 1]
        if self.modes[0] == 0:
            a = self.ops[a]
        b = self.ops[self.ptr + 2]
        if self.modes[1] == 0:
            b = self.ops[b]
        c = self.ops[self.ptr + 3]
        maybe_print(f"a={a} b={b} c={c}")
        self.ops[c] = a * b
        self.ptr += 4

    def op3(self):
        maybe_print(f"OP IN (*{self.ptr}) {self.ops[self.ptr:self.ptr+1]}")
        addr = self.ops[self.ptr + 1]
        self.mem[addr] = int(input('> '))
        self.ptr += 2

    def op4(self):
        maybe_print(f"OP OUT (*{self.ptr}) {self.ops[self.ptr:self.ptr+1]}")
        print("MEM", self.mem[self.ops[self.ptr + 1]])
        self.ptr += 2

    def halt(self):
        self.halted = True
        return self.ops[0]

