import re
from aoc import mult


def day16(data):
    data = iter(data)
    fields = {}
    for row in data:
        if not row:
            break
        key, a_min, a_max, b_min, b_max = re.match(
            r"([^:]*): (\d*)-(\d*) or (\d*)-(\d*)", row).groups()
        fields[key] = (int(a_min), int(a_max), int(b_min), int(b_max))

    next(data)
    mine = [int(x) for x in next(data).split(",")]
    next(data)
    next(data)
    tickets = [
        [int(x) for x in t.split(",")]
        for t in data
    ]

    valid = []
    error_rate = 0
    for t in tickets:
        is_valid = True
        for x in t:
            for a_min, a_max, b_min, b_max in fields.values():
                if (x >= a_min and x <= a_max) or (x >= b_min and x <= b_max):
                    break
            else:
                error_rate += x
                is_valid = False
        if is_valid:
            valid.append(t)

    field_map = [set(fields.keys()) for _ in range(len(mine))]
    for t in valid:
        for i, x in enumerate(t):
            for field_name in list(field_map[i]):
                a_min, a_max, b_min, b_max = fields[field_name]
                if not (x >= a_min and x <= a_max) \
                        and not (x >= b_min and x <= b_max):
                    field_map[i].remove(field_name)
    # reduce
    for _ in range(len(field_map)):
        for i, field_set in enumerate(field_map):
            if len(field_set) == 1:
                for j in range(len(field_map)):
                    if i != j:
                        field_map[j].discard(next(iter(field_set)))
    res = [mine[i]
           for i, fieldset in enumerate(field_map)
           if next(iter(fieldset)).startswith("departure")]

    return error_rate, mult(res)
