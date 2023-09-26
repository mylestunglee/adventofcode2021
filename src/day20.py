import sys

def read_puzzle(filename):
    with open(filename) as file:
        puzzle_str = file.read()
    enhancement_str, image_str = puzzle_str.split('\n\n')
    enhancement = [1 if char == '#' else 0 for char in enhancement_str]
    assert(len(enhancement) == 512)
    image = [[1 if char == '#' else 0 for x, char in enumerate(row)] for y, row in enumerate(image_str.splitlines())]
    return enhancement, image, 0

def calc_binary_neighbours(image, background, x, y):
    width = len(image[0])
    height = len(image)
    total = 0
    for dx in range(-1, 2):
        for dy in range(-1, 2):
            u = x + dx
            v = y + dy
            cell = background if u < 0 or u >= width or v < 0 or v >= height else image[v][u]
            if cell:
                total += 2 ** (3 * (1 - dy) + (1 - dx))
    return total

def enhance(image, background, enhancement):
    width = len(image[0])
    height = len(image)
    projection = [[enhancement[calc_binary_neighbours(image, background, x, y)] for x in range(-1, width + 2)] for y in range(-1, height + 2)]
    return projection, enhancement[-background]

def count_lit(image):
    return sum(map(sum, image))

enhancement, image, background = read_puzzle(sys.argv[1])

for i in range(50):
    if i == 2:
        print('part1', count_lit(image))
    image, background = enhance(image, background, enhancement)

print('part2', count_lit(image))
