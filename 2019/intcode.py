import logging
import pyperclip as clip

logging.basicConfig(
    level=logging.DEBUG,
    format="%(asctime)s [%(levelname)s] %(message)s",
    handlers=[
        logging.FileHandler("debug.log"),
        logging.StreamHandler()
    ],
)
class IntCode:
    """Represents an IntCode machine"""
    def __init__(self, opcodes, **kwargs):
        """
        Create a new IntCode machine with opcodes.

        Keyword Arguments
        -----------------
        inputs : list [int]
            Inputs to send to the machine for Opcode 3

        level : logging level
            What logging to use for the IntCode machine's debug messages

        """
        self.ops = [int(i) for i in opcodes]
        self.ptr = 0
        self.halted = False
        self.mem = dict()
        self.modes = []
        self.inputs = kwargs.get("inputs", [])
        self.outputs = []
        self.logger = logging.getLogger("intcode")
        self.logger.setLevel(kwargs.get("level", logging.DEBUG))
        self.logger.debug("INPUTS %s", self.inputs)

    def run(self):
        """
        Run the IntCode machine until it halts or errors.
        """
        while not self.halted:
            self.logger.debug("STEP: %s", self.ops[self.ptr:self.ptr+4])
            self.one_step()
        return self.ops[0]

    def parse_instr(self, s):
        """
        Read an opcode into list of params and codes.

        Non-existent params are given value 0.
        """
        s = str(s).rjust(5, "0")
        return [int(i) for i in str(s).rjust(5, "0")]

    def one_step(self):
        """
        Read and evaluate the next opcode.
        """
        if self.halted:
            return self.ops[0]
        intcode = self.parse_instr(self.ops[self.ptr])
        self.ptr += 1

        instrs = {
            1: self.add,
            2: self.mul,
            3: self.save,
            4: self.read,
            5: self.jump_true,
            6: self.jump_false,
            7: self.less_than,
            8: self.equals,
            99: self.halt,
        }

        opkey = intcode[-2] * 10 + intcode[-1]
        try:
            op = instrs[opkey](intcode)
        except Exception as E:
            print(instrs[opkey], intcode)
            raise Exception(f"BAD OP CODE `{opkey}` - ops {self.ops[self.ptr:self.ptr+5]}")
        if self.ptr >= len(self.ops):
            raise Exception("Pointer past end of operations")

    def get(self, offset, mode=0):
        """
        Read val from memory

        if mode == I (immediate), use this as value
        else, treat it as first memory address, and return ops[val]
        """
        if mode == 0:
            return int(self.ops[self.ops[self.ptr + offset]])
        return int(self.ops[self.ptr + offset])
    
    def set(self, offset, value):
        """
        Set a memory address to a value.
        """
        self.ops[self.ops[self.ptr + offset]] = value

    def jump_true(self, intcode):
        """
        Jump the opcode pointer if a value is non-zero.
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        if first != 0:
            self.ptr = second
        else:
            self.ptr += 2

    def jump_false(self, intcode):
        """
        Jump the opcode pointer if a value is zero.
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        if first == 0:
            self.ptr = second
        else:
            self.ptr += 2

    def less_than(self, intcode):
        """
        Set a memory value to 1 or 0 based on <.
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        self.set(2, 1 if first < second else 0)
        self.ptr += 3

    def equals(self, intcode):
        """
        Set a memory address to 1 or 0 based on equals.
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        self.set(2, 1 if first == second else 0)
        self.ptr += 3

    def add(self, intcode):
        """
        ops[c] = first + second
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        store_at = int(self.ops[self.ptr+2])
        self.logger.debug("ADD  %d + %d => ops[%s]", first, second, store_at)
        self.set(2, first + second)
        self.ptr += 3

    def mul(self, intcode):
        """
        ops[c] = first * second
        """
        first = self.get(0, intcode[2])
        second = self.get(1, intcode[1])
        store_at = int(self.ops[self.ptr+2])
        self.logger.debug("MULT %d * %d => ops[%d]", first, second, store_at)
        self.set(2, first * second)
        self.ptr += 3

    def save(self, intcode):
        """
        ops[first] = ...input...
        """
        store_at = int(self.ops[self.ptr])
        popped = self.inputs.pop(0)
        self.logger.debug("SAVE %s => ops[%s]", popped, store_at)
        self.logger.info("IN %s", popped)
        self.set(0, popped)
        self.ptr += 1

    def read(self, intcode):
        """
        OUTPUT = ops[first]
        """
        mem_addr = self.get(0, intcode[2])
        self.logger.debug("READ ops[%s] => %s", self.ops[self.ptr], mem_addr)
        self.logger.info("OUT %s", mem_addr)
        self.outputs.append(mem_addr)
        clip.copy(mem_addr)
        self.ptr += 1

    def last_output(self):
        """
        Return the last output the program transmitted.
        """
        return self.outputs[-1]

    def halt(self, _intcode):
        """
        STOP PROGRAM and output ops[0]
        """
        self.halted = True
        return self.ops[0]

