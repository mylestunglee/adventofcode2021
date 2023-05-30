import sys
import numpy as np
from pathfinding.core.diagonal_movement import DiagonalMovement
from pathfinding.core.grid import Grid
from pathfinding.finder.a_star import AStarFinder

matrix = []
with open(sys.argv[1]) as file:
    for line in file.readlines():
        matrix.append(list(map(int, line.strip())))

def find_minimal_risk_path(matrix):
    grid = Grid(matrix=matrix)
    start = grid.node(0, 0)
    end = grid.node(grid.width - 1, grid.height - 1)
    finder = AStarFinder(diagonal_movement=DiagonalMovement.never)
    path, runs = finder.find_path(start, end, grid)
    return sum(matrix[y][x] for x, y in path) - matrix[0][0]

def repeat_map(matrix, tile):
    return [[(cell + x + y - 1) % 9 + 1 for x in range(tile) for cell in row] for y in range(tile) for row in matrix]

def solve(tile):
    return find_minimal_risk_path(repeat_map(matrix, tile))

print("part1={}".format(solve(1)))
print("part2={}".format(solve(5)))
