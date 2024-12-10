import sys

with open(sys.argv[1], "r") as file:
    # links coordinate to height
    heightmap = {
        i + j * 1j: int(value)
        for i, line in enumerate(file)
        for j, value in enumerate(line.strip())
    }


# Creates Graph
possible_moves = [1, -1, 1j, -1j]
graph = dict()
for pos in heightmap:
    moves = [
        pos + i
        for i in possible_moves
        if pos + i in heightmap and heightmap[pos + i] == heightmap[pos] + 1
    ]
    graph[pos] = moves


# Depth First Search function
def dfs(graph, start, end, visited=None):
    if visited is None:
        visited = set()

    # if we find the point we want, return 1 to end the current search
    if start == end:
        return 1

    visited.add(start)
    # otherwise, assume no paths start from the current node
    path_count = 0

    for neighbour in graph[start]:
        # if we've not visited any of the neighbours, go to them
        if neighbour not in visited:
            # and update any paths that hit the end from that point
            path_count += dfs(graph, neighbour, end, visited)

    # remove our start point so we can continue from higher up
    visited.remove(start)

    return path_count


# Find start and end nodes
starts = []
ends = []

for node in heightmap:
    if heightmap[node] == 0:
        starts.append(node)
    elif heightmap[node] == 9:
        ends.append(node)


# Loop through every start and end and sum up total paths
total_paths = 0
is_path = 0

for start in starts:
    for end in ends:
        paths = dfs(graph, start, end)
        total_paths += paths
        if paths:
            is_path += 1


print(f"Part 1 {is_path}")
print(f"Part 2 {total_paths}")
