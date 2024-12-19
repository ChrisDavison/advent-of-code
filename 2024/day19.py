from utility import parse, atoms, paragraphs


def solve(towels, pattern, depth=0):
    if len(pattern) == 1:
        if pattern[0] in towels:
            return True
        return False
    if pattern in towels:
        return True
    if pattern[0] in towels:
        return solve(towels, pattern[1:], depth + 1)
    return False


def run(filename):
    towels, patterns = parse(filename, atoms, section_by=paragraphs, show=0)
    longest_towel = max(len(t) for t in towels)

    patterns_covered = 0
    for p in patterns:
        tokens_covered = [False for _ in range(len(p))]
        # print("---")
        towels_used = set()
        for i, ch in enumerate(p):
            # print(" " * i + "v")
            # print(p)
            # print()
            if ch in towels:
                tokens_covered[i] = True
            for j in range(longest_towel):
                sub = p[i : i + j + 1]
                if sub in towels:
                    towels_used.add(sub)
                    for k in range(i, i + len(sub)):
                        # print(k)
                        tokens_covered[k] = True
            if all(tokens_covered):
                patterns_covered += 1
                break
        print(p, all(tokens_covered), "|".join(towels_used))
        # break
    print(patterns_covered)
