import ast
import sys

def snailfish_add(x, y):
    return snailfish_reduce([x, y])

def snailfish_leftmost_add(x, v):
    assert(type(v) is int)

    if v == 0:
        return x

    if type(x) is int:
        return x + v

    return [snailfish_leftmost_add(x[0], v), x[1]]

def snailfish_rightmost_add(x, v):
    assert(type(v) is int)

    if v == 0:
        return x

    if type(x) is int:
        return x + v

    return [x[0], snailfish_rightmost_add(x[1], v)]

def get_max_depth(x):
    if type(x) is int:
        return 0
    else:
        return max(get_max_depth(x[0]), get_max_depth(x[1])) + 1

def snailfish_explode_recursive(x, explode_depth):
    if type(x) is int:
        return False, x, 0, 0

    if explode_depth == 0:
        return True, 0, x[0], x[1]


    reduced, y, left, right = snailfish_explode_recursive(x[0], explode_depth - 1)
    if reduced:
        return True, [y, snailfish_leftmost_add(x[1], right)], left, 0

    reduced, y, left, right = snailfish_explode_recursive(x[1], explode_depth - 1)
    if reduced:
        return True, [snailfish_rightmost_add(x[0], left), y], 0, right

    return False, x, 0, 0

def snailfish_explode(x):
    max_depth = get_max_depth(x)
    reduced, x, _, _ = snailfish_explode_recursive(x, max(max_depth - 1, 4))
    return reduced, x

def snailfish_split(x):
    if type(x) is int:
        if x >= 10:
            if x % 2 == 0:
                return True, [x // 2, x // 2]
            else:
                return True, [x // 2, x // 2 + 1]
        return False, x

    reduced, y = snailfish_split(x[0])
    if reduced:
        return True, [y, x[1]]

    reduced, y = snailfish_split(x[1])
    if reduced:
        return True, [x[0], y]

    return False, x

def snailfish_reduce(x):
    reduced = True
    while reduced:
        reduced, x = snailfish_explode(x)
        while reduced:
            reduced, x = snailfish_explode(x)
        reduced, x = snailfish_split(x)
    return x

def snailfish_magnitude(x):
    if type(x) is int:
        return x
    
    return 3 * snailfish_magnitude(x[0]) + 2 * snailfish_magnitude(x[1])

def read_problem():
    numbers = []

    with open(sys.argv[1]) as file:
        for line in file.readlines():
            numbers.append(ast.literal_eval(line.strip()))

    return numbers

def solve1(numbers):
    if not numbers:
        return

    total = numbers[0]
    for number in numbers[1:]:
        total = snailfish_add(total, number)

    return snailfish_magnitude(total)

def solve2(numbers):
    return max(snailfish_magnitude(snailfish_add(x, y)) for x in numbers for y in numbers if x != y)

numbers = read_problem()
print('part1={}'.format(solve1(numbers)))
print('part2={}'.format(solve2(numbers)))
