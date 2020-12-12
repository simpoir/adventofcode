dirs = (
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
)


def gen(data):
    graph = {}
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == "L":
                branches = graph.setdefault((x, y), [])
                for dx, dy in dirs:
                    xx = x + dx
                    yy = y + dy
                    try:
                        if xx >= 0 and yy >= 0 and data[yy][xx] == "L":
                            branches.append((xx, yy))
                    except IndexError:
                        pass
    return graph


def gen2(data):
    graph = {}
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == "L":
                branches = graph.setdefault((x, y), [])
                for dx, dy in dirs:
                    try:
                        xx = x + dx
                        yy = y + dy
                        while xx >= 0 and yy >= 0:
                            try:
                                if data[yy][xx] == "L":
                                    branches.append((xx, yy))
                                    raise StopIteration
                            except IndexError:
                                raise StopIteration
                            xx += dx
                            yy += dy
                    except StopIteration:
                        continue
    return graph


def solve(graph, thresh=4):
    # Ok, this solution will sound look crazy, so allow me to explain.
    # Instead or iterating our little game of life, we solve it with maths.
    # This builds on the fact there are no empty seat from the start, thus
    # all seats are full on the second step. We then eliminate known values.
    #
    # 1. Generate a graph of seat coordinates to adjacent seat coordinates.
    # 2. Store which seats we know hold someone.
    settled = set()
    # 3. run until there are no unknown-state seats.
    while graph:
        # 4. keep a bin for to avoid removing items while iterating
        rubbish = set()
        # 5. for each position...
        for pos, branches in graph.items():
            # ... if there are at less N neighbors
            if len(branches) < thresh:
                # ... that seat will stay full
                settled.add(pos)
                # ... so we don't need to check it again
                rubbish.add(pos)
                # also, we don't neeed to check the neighbors, as there will
                # never be 0 seat around them.
                for b in branches:
                    rubbish.add(b)
        for p in rubbish:
            for b in graph.pop(p):
                graph[b].remove(p)
    # This makes the critical assumption that there is a single solution.
    return settled


def day11(data):
    return len(solve(gen(data))), len(solve(gen2(data), thresh=5))
