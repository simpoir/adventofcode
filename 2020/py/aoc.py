#!/usr/bin/python3
import re
from functools import reduce
from itertools import chain, combinations, groupby

MAGIC = 2020


def mult(iterable):
    return reduce(lambda a, b: a * b, iterable)


def day01(data):
    def do(domain, count):
        for x in combinations(domain, count):
            if sum(x) == MAGIC:
                return mult(x)

    data = [int(i) for i in data]
    return do(data, 2), do(data, 3)


def day02(data):
    def test(line):
        lo, hi, c, entry = re.match(r"(\d+)-(\d+) (.): (.*)$", line).groups()
        count = entry.count(c)
        return count >= int(lo) and count <= int(hi)

    def test2(line):
        p1, p2, c, entry = re.match(r"(\d+)-(\d+) (.): (.*)$", line).groups()
        return (entry[int(p1) - 1] == c) ^ (entry[int(p2) - 1] == c)

    return (
        sum([test(line) for line in data]),
        sum([test2(line) for line in data]),
    )


def day03(data):
    def do(data, left, down):
        return sum(
            [
                row[int(i / down) * left % len(row)] == "#"
                for i, row in enumerate(data)
                if i % down == 0
            ]
        )

    data = list(data)

    return (
        do(data, 3, 1),
        mult([do(data, *a) for a in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))]),
    )


def day04(data):
    required = set(("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"))  # no cid
    arr = [
        dict(x.split(":", 1) for x in " ".join(v).split(" "))
        for k, v in groupby(data, lambda x: not x)
        if not k
    ]
    return (
        sum([required.issubset(x) for x in arr]),
        sum(
            [
                required.issubset(x)
                and int(x["byr"]) in range(1920, 2003)
                and int(x["iyr"]) in range(2010, 2021)
                and int(x["eyr"]) in range(2020, 2031)
                and (
                    int(x["hgt"][:-2]) in range(150, 194)
                    if x["hgt"].endswith("cm")
                    else int(x["hgt"][:-2]) in range(59, 77)
                    if x["hgt"].endswith("in")
                    else False
                )
                and re.fullmatch("#[0-9a-f]{6}", x["hcl"]) is not None
                and x["ecl"]
                in ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
                and re.fullmatch(r"\d{9}", x["pid"]) is not None
                for x in arr
            ]
        ),
    )


def day05(data):
    ids = set(
        [
            int(pos[:7], base=2) * 8 + int(pos[-3:], base=2)
            for pos in map(
                lambda pos: "".join(
                    ["1" if x in ("B", "R") else "0" for x in pos]
                ),
                data,
            )
        ]
    )

    return max(ids), next(
        iter(
            i
            for i in range(999)
            if i not in ids and i + 1 in ids and i - 1 in ids
        )
    )


def day06(data):
    groups = [
        len(set(chain(*g)))
        for k, g in groupby(data, lambda x: len(x) > 0)
        if k
    ]
    groups2 = [
        len(reduce(lambda x, y: set(x).intersection(set(y)), g))
        for k, g in groupby(data, lambda x: len(x) > 0)
        if k
    ]
    return sum(groups), sum(groups2)


def day07(data):
    containers = {
        k: v for k, v in map(lambda x: x.split(" bags contain "), data)
    }

    def expand(this):
        opts = [k for k, v in containers.items() if this in v]
        return chain(opts, *[expand(o) for o in opts])

    def nest(this):
        row = containers[this]
        total = 0
        for match in re.finditer(r"(\d+) ([a-z ]+) bag", row):
            count, bag = int(match.groups()[0]), match.groups()[1]
            total += count + count * nest(bag)
        return total

    return len(set(expand("shiny gold"))), nest("shiny gold")


def day08(data):
    def vm(prog):
        acc = cr = 0
        loops = set()
        while cr not in loops:
            loops.add(cr)
            if cr >= len(prog):
                return acc
            op, arg = prog[cr]
            if op == "jmp":
                cr += arg
            else:
                if op == "acc":
                    acc += arg
                cr += 1
        raise RuntimeError(acc)  # loop exception

    prog = tuple((i[:3], int(i[4:])) for i in data)
    try:
        vm(prog)
    except RuntimeError as e:
        (res1,) = e.args

    for i in range(len(prog)):
        try:
            tgt = {"jmp": "nop", "nop": "jmp"}[prog[i][0]]
        except KeyError:
            continue
        prog2 = prog[:i] + ((tgt, prog[i][1]),) + prog[i + 1 :]
        try:
            res2 = vm(prog2)
        except RuntimeError:
            continue
        break

    return res1, res2


def day09(data, preamble=25):
    data = list([int(x) for x in data])
    for i, res1 in enumerate(data[preamble:]):
        for a, b in combinations(data[i : i + preamble], 2):
            if a + b == res1:
                break
        else:
            break

    q = []
    while True:
        s = sum(q)
        if s < res1:
            q.append(data.pop(0))
        elif s > res1:
            q.pop(0)
        else:  # sum(q) == res1:
            res2 = min(q) + max(q)
            break

    return res1, res2


def day10(data):
    prev = 0
    dist = {1: 0, 2: 0, 3: 1}
    data = list(sorted([int(x) for x in data]))
    for x in data:
        dist[x - prev] += 1
        prev = x
    res = dist[1] * dist[3]

    paths = {data[-1] + 3: 1}
    for x in reversed(data):
        paths[x] = sum([paths[x + i] for i in range(1, 4) if (x + i) in paths])

    res2 = sum([paths[i] for i in range(1, 4) if i in paths])
    return res, res2


def day11(data):
    dirs = ((-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0),
            (1, 1))

    def gen2(data):
        graph = {}
        for y, line in enumerate(data):
            for x, c in enumerate(line):
                if c == 'L':
                    branches = graph.setdefault((x, y), [])
                    for dx, dy in dirs:
                        try:
                            xx = x + dx
                            yy = y + dy
                            while xx >= 0 and yy >= 0:
                                try:
                                    if data[yy][xx] == 'L':
                                        branches.append((xx, yy))
                                        raise StopIteration
                                except IndexError:
                                    raise StopIteration
                                xx += dx
                                yy += dy
                        except StopIteration:
                            continue
        return graph

    def gen(data):
        graph = {}
        for y, line in enumerate(data):
            for x, c in enumerate(line):
                if c == 'L':
                    branches = graph.setdefault((x, y), [])
                    for dx, dy in dirs:
                        xx = x+dx
                        yy = y+dy
                        try:
                            if xx >= 0 and yy >= 0 and data[yy][xx] == 'L':
                                branches.append((xx, yy))
                        except IndexError:
                            pass
        return graph

    def solve(graph, thresh=4):
        settled = set()
        while graph:
            rubbish = set()
            for pos, branches in graph.items():
                if len(branches) < thresh:
                    settled.add(pos)
                    rubbish.add(pos)
                    for b in branches:
                        rubbish.add(b)
            # prune after tagging
            for p in rubbish:
                for b in graph.pop(p):
                    graph[b].remove(p)
        return settled

    return len(solve(gen(data))), len(solve(gen2(data), thresh=5))
