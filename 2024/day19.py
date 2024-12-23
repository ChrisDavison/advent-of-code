from functools import cache
from utility import parse, atoms, paragraphs


def possible(towel, patterns):
    return towel == "" or any(
        towel.startswith(pat) and possible(towel[len(pat) :], patterns)
        for pat in patterns
    )


@cache
def count_possible(towel, patterns):
    return (
        1
        if towel == ""
        else sum(
            count_possible(towel[len(pat) :], patterns)
            for pat in patterns
            if towel.startswith(pat)
        )
    )


def run(filename):
    patterns, towels = parse(filename, atoms, section_by=paragraphs, show=0)

    patterns_covered = 0
    for towel in towels:
        if possible(towel, patterns):
            patterns_covered += 1
    print(patterns_covered)


def run2(filename):
    patterns, towels = parse(filename, atoms, section_by=paragraphs, show=0)

    patterns_covered = 0
    for towel in towels:
        pos = count_possible(towel, patterns)
        patterns_covered += pos
    print(patterns_covered)


run("19")
run2("19")
