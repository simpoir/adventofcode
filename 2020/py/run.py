#!/usr/bin/python3
import argparse
import json
import os
import sys

import aoc

MAGIC = os.path.basename(os.path.abspath(".."))


def run(day, args):
    suffix = "test" if args.test else "txt"
    print(f"Advent of code {MAGIC}, day{day:02}:")
    with open(f"../data/day{day:02}.{suffix}") as text:
        extras = {}
        if args.test:
            extras = json.loads(text.readline())
            expected = extras.pop("expected")
        res = days[day](text.read().strip().splitlines(), **extras)
        res = tuple(str(r) for r in res)
        print(res)
        if args.test and res != tuple(expected):
            print(f"Nope! That's not it.\n Expected {expected}")
            sys.exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="run test")
    parser.add_argument("--all", action="store_true", help="run all")
    parser.add_argument("--list", action="store_true", help="list days")
    parser.add_argument("day", nargs="?", type=int)
    args = parser.parse_args()

    days = {}
    for d in range(31):
        name = f"day{d:02}"
        try:
            days[d] = getattr(aoc, name)
        except AttributeError:
            try:
                days[d] = getattr(
                    __import__(f"aoc.{name}", fromlist=name), name
                )
            except ImportError:
                pass

    if args.list:
        tuple(print(d) for d in days)
        sys.exit(0)

    day = args.day or int(max(days))
    if args.all:
        for d in days:
            run(d, args)
            print()
    else:
        run(day, args)
