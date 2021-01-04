import math
from functools import reduce


TESTING = False


def mult(iterable):
    return reduce(lambda a, b: a * b, iterable)


def day01(data):
    def do(data):
        return sum([math.floor(i / 3) - 2 for i in data])

    data = [int(i) for i in data]
    res1 = do(data)
    res2 = 0
    for fuel in data:
        while True:
            fuel = do([fuel])
            if fuel <= 0:
                break
            res2 += fuel
    return res1, res2


def day02(data):
    def fun(prog, x, y):
        prog = list(prog)
        prog[1:3] = [x, y]
        i = 0
        while True:
            if prog[i] == 99:
                break
            op, a, b, dest, *_ = prog[i:]
            if op == 1:
                prog[dest] = prog[a] + prog[b]
            elif op == 2:
                prog[dest] = prog[a] * prog[b]
            else:
                print(prog)
                raise RuntimeError(op)
            i += 4
        return prog[0]

    prog = tuple(int(i) for i in data[0].split(","))
    x, y = (12, 2) if not TESTING else (prog[1], prog[2])

    res1 = fun(prog, x, y)
    for x in range(99):
        for y in range(99):
            if fun(prog, x, y) == prog[-1]:
                return res1, x * 100 + y
    else:
        raise RuntimeError("noes")


def day03(data):
    def expand(ops):
        x, y, z = (0, 0, 0)
        for op in ops.split(","):
            for i in range(int(op[1:])):
                if op[0] == "R":
                    x += 1
                elif op[0] == "L":
                    x -= 1
                elif op[0] == "U":
                    y += 1
                else:
                    y -= 1
                z += 1
                yield x, y, z

    a = {(x, y): z for x, y, z in expand(data[0])}
    b = {(x, y): z for x, y, z in expand(data[1])}
    c = min([abs(x) + abs(y) for (x, y) in a if (x, y) in b])
    d = min([a[k] + b[k] for k in a if k in b])
    return c, d


def day04(data):
    def test(p):
        s = str(p)
        return "".join(sorted(s)) == s and len(set(s)) < 6

    def test2(p):
        s = str(p)
        return (
            "".join(sorted(s)) == s
            and len(set(s)) < 6
            and any([2 == s.count(i) for i in s])
        )

    res1 = [test(i) for i in range(int(data[0][:6]), int(data[0][7:]))]
    res2 = [test2(i) for i in range(int(data[0][:6]), int(data[0][7:]))]
    return sum(res1), sum(res2)
