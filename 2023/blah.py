#!/usr/bin/env python3
import re

class Solution:
    year = 2023
    day = 3
    input: str
    data: list
    re = re.compile(r"\d+")

    def __init__(self, folder="."):
        with open(f"input/day03") as f:
            self.input = f.read()
        self.prepare_data()

    def prepare_data(self):
        self.data = self.input.strip().split("\n")

    def find_numbers(self, line):
        """Generator to find numbers on each line"""
        end = 0
        while m := self.re.search(line, end):
            start = max(m.start() - 1, 0)
            end = m.end() + 1
            yield start, min(end, len(line) - 1), int(m[0])

    def get_coords(self, y, start, end, check):
        """Get the coords if the number has any symbols adjacent to it"""
        return [
            # x is index number of sliced part so need to add ``start`` to
            # find the correct ``x`` coord.
            f"{x + start}-{ln}"
            for ln in [max(y - 1, 0), y, min(y + 1, len(self.data) - 1)]
            for x, char in enumerate(self.data[ln][start:end])
            if check(char)
        ]

    def part1(self):
        """Part 1

        Sum of all numbers near any symbol except ``.``
        """
        return sum(
            number
            for y, line in enumerate(self.data)
            for start, end, number in self.find_numbers(line)
            if any(self.get_coords(y, start, end, lambda c: c not in "1234567890."))
        )

    def part2(self):
        """Part 2

        Find the 2 number adjacent to a ``*`` and multiply them.
        Solution is the result of those.
        """
        # Find the numbers with ``*`` near to them
        numbers = [
            (number, coord)
            for y, line in enumerate(self.data)
            for start, end, number in self.find_numbers(line)
            for coord in self.get_coords(y, start, end, lambda c: c == "*")
        ]

        checked = []

        def append(coord, number):
            checked.append(coord)
            return number

        return sum(
            append(coord1, number1 * number2)
            for i, (number1, coord1) in enumerate(numbers)
            for j, (number2, coord2) in enumerate(numbers)
            if j != i and coord1 == coord2 and coord1 not in checked
        )


if __name__ == "__main__":
    s = Solution()
    print(f"AoC [{s.year} - Day {s.day}] Part 1: {s.part1()}")
    print(f"AoC [{s.year} - Day {s.day}] Part 2: {s.part2()}")
