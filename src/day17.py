import sys
import re

with open(sys.argv[1]) as file:
    file_data = file.read().strip()

def simulate_max_y(velocity_x, velocity_y, bounds):
    x = 0
    y = 0
    max_y = -9999
    while x <= bounds[1] and y >= bounds[2]:
        x += velocity_x
        y += velocity_y
        max_y = max(max_y, y)
        if velocity_x > 0:
            velocity_x -= 1
        velocity_y -= 1
        
        if bounds[0] <= x <= bounds[1] and bounds[2] <= y <= bounds[3]:
            return max_y

    return -9999

def solve1(file_data):
    pattern = re.compile('target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)')
    bounds = list(map(int, pattern.match(file_data).groups()))
    max_y = -9999
    for velocity_x in range(1000):
        for velocity_y in range(1000):
            max_y = max(max_y, simulate_max_y(velocity_x, velocity_y, bounds))

    return max_y

def solve2(file_data):
    pattern = re.compile('target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)')
    bounds = list(map(int, pattern.match(file_data).groups()))
    count = 0

    for velocity_x in range(-500, 500):
        for velocity_y in range(-500, 500):
            if simulate_max_y(velocity_x, velocity_y, bounds) != -9999:
                #print(velocity_x, velocity_y)
                count += 1
    return count

#print('part1={}'.format(solve1(file_data)))
print('part2={}'.format(solve2(file_data)))

