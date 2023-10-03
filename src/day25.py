import fileinput

def read_grid():
    grid = []
    for line in fileinput.input():
        grid.append(list(line.strip()))
    return grid

def process_right(old_grid):
    width, height = len(old_grid[0]), len(old_grid)
    new_grid = []
    for y in range(height):
        line = []
        for x in range(width):
            if old_grid[y][x] == '.' and old_grid[y][(x + width - 1) % width] == '>':
                cell = '>'
            elif old_grid[y][(x + 1) % width] == '.' and old_grid[y][x] == '>':
                cell = '.'
            else:
                cell = old_grid[y][x]
            line.append(cell)
        new_grid.append(line)
    return new_grid

def process_down(old_grid):
    width, height = len(old_grid[0]), len(old_grid)
    new_grid = []
    for y in range(height):
        line = []
        for x in range(width):
            if old_grid[y][x] == '.' and old_grid[(y + height - 1) % height][x] == 'v':
                cell = 'v'
            elif old_grid[(y + 1) % height][x] == '.' and old_grid[y][x] == 'v':
                cell = '.'
            else:
                cell = old_grid[y][x]
            line.append(cell)
        new_grid.append(line)
    return new_grid

grid = read_grid()
for i in range(100000):
    next_grid = process_right(grid)
    next_grid = process_down(next_grid)
    if grid == next_grid:
        print(i + 1)
        break
    grid = next_grid
    # print(i)
    # print('\n'.join(''.join(line) for line in grid))
