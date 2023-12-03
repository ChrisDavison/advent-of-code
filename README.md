# Advent of Code

Solutions to all problems of the [Advent of Code](https://adventofcode.com/2020) christmas programming puzzle set.

- 2020, done in *Rust*

## Tips

### Iterate over a string and get a number from a sequence *in one pass*

i.e. if iterating over `abc123def456`, how do you parse the numbers 123 and 456 purely inline through the iteration?

```python
G = [[c for c in line] for line in lines]  # grid
for row in range(len(G)):
    n = 0
    for col in range(len(G[i])):
        if G[row][col].isdigit():
            # First pass, we see 1. n = 0*10 + 1 == 1
            # .. then, 2. n = 10*1 + 2 = 12
            # .. then, 3. n = 10*12 + 3 = 123
            n = n*10 + int(G[row][col])
            # next time, 4 => 40 + 4 => 450 + 6
        else:
            n = 0
```
