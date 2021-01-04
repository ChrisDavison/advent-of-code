#!/usr/bin/env python3
from prelude import *
from collections import defaultdict, Counter
from intcode import IntCode
import pyperclip as clip
import logging
import networkx as nx

SAMPLE = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"


if __name__ == "__main__":
    # part1
    m0 = IntCode(SAMPLE.split(","), inputs=[0])
    m = IntCode(SAMPLE.split(","), inputs=[4])
    # m2 = IntCode(SAMPLE.split(","), inputs=[3])
    # m3 = IntCode(SAMPLE.split(","), inputs=[2])
    # m4 = IntCode(SAMPLE.split(","), inputs=[1])
    # m5 = IntCode(SAMPLE.split(","), inputs=[0])
    m0.run()
    m.run()
    # m2.run()
    # m3.run()
    # m4.run()
    # m5.run()
    p1 = m.last_output()
    print(f"2019 7.1 -> {p1}")
    clip.copy(str(p1))

    # part2
    p2 = None
    print(f"2019 7.2 -> {p2}")
    clip.copy(str(p2))

