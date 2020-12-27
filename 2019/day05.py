#!/usr/bin/env python3
from prelude import *
from collections import defaultdict, Counter
from intcode import IntCode
import logging


if __name__ == "__main__":

    data = [l for l in open('input/05').read().split(',')]

    m = IntCode(data.copy(), inputs=[1], level=logging.WARNING)
    m.run()
    print("2019 5.1 ->", m.last_output())

    m = IntCode(data.copy(), inputs=[5], level=logging.WARNING)
    m.run()
    print("2019 5.2 ->", m.last_output())

