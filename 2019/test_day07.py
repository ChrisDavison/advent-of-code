#!/usr/bin/env python3
import day01
from intcode import IntCode

def test_day07p1e1():
    code = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".split(",")
    inputs = [ 4, 3, 2, 1, 0]
    m = IntCode(code, inputs=[inputs[0]])
    m.run()

    # m = IntCode(code, inputs=[m.last_output()])
    # m.run()

    # m = IntCode(code, inputs=[m.last_output()])
    # m.run()

    # m = IntCode(code, inputs=[m.last_output()])
    # m.run()

    # m = IntCode(code, inputs=[m.last_output()])
    # m.run()

    assert 43210 == m.last_output()

# def test_day07p1e1():
#     code = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".split(",")
#     m = IntCode(code, inputs=[0,1,2,3,4])
#     m.run()
#     assert 54321 == m.last_output()

