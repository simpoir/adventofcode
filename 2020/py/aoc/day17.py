import itertools


def run(initial, dimensions):
    cubes = {
        (x, y) + (0,) * (dimensions - 2): 0
        for y, l in enumerate(initial)
        for x, c in enumerate(l)
        if c == "#"
    }
    adj = [
        p
        for p in itertools.product(range(-1, 2), repeat=dimensions)
        if p != (0,) * dimensions
    ]

    for _ in range(6):
        cubes1 = {}
        for p in cubes:
            for inc in adj:
                pp = tuple(p[i] + inc[i] for i in range(dimensions))
                cubes1[pp] = cubes1.get(pp, 0) + 1
        cubes = {
            p: count
            for p, count in cubes1.items()
            if count == 3 or (count == 2 and p in cubes)
        }
    return len(cubes)


def day17(data):
    return run(data, 3), run(data, 4)
