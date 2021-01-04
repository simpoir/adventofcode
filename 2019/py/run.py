#!/usr/bin/python3
import argparse
import os
import sys

import aoc

MAGIC = os.path.basename(os.path.abspath('..'))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="run test")
    parser.add_argument("--list", action="store_true", help="list days")
    parser.add_argument("day", nargs="?", type=int)
    args = parser.parse_args()

    days = {
        d[3:]: getattr(aoc, d) for d in dir(aoc) if d.startswith("day")
    }
    if args.list:
        tuple(print(d) for d in days)
        sys.exit(0)

    day = args.day or int(max(days))
    day = f"{day:02}"

    suffix = "test" if args.test else "txt"
    aoc.TESTING = args.test

    print(f"Advent of code {MAGIC}, day{day}:")
    with open(f"../data/day{day}.{suffix}") as text:
        if args.test:
            expected = (text.readline().strip(), text.readline().strip())
        res = days[day](text.read().strip().splitlines())
        res = tuple(str(r) for r in res)
        print(res)
        if args.test and res != expected:
            print(f"Nope! That's not it.\n Expected {expected}")
            sys.exit(1)
