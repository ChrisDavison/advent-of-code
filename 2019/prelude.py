import numpy as np
import pyperclip as clip
from collections import *
from itertools import *
from datetime import datetime

dt_str = lambda start: "({}ms)".format((datetime.now() - start).microseconds)
now = datetime.now

def ms_time(f, *args, **kwargs):
    start = datetime.now()
    val = f(*args, **kwargs)
    delta = (datetime.now() - start).microseconds
    return (val, delta)


def timed(funcname, f, *args, **kwargs):
    ans, delta = ms_time(f, *args, **kwargs)
    print("{} → {} ({:.1f}ms)".format(funcname, ans, delta / 1000))
