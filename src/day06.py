import numpy as np
import sys

STATE_COUNT = 9
initial = np.zeros(STATE_COUNT, dtype=int)
with open(sys.argv[1]) as file:
    for token in file.read().strip().split(","):
        initial[int(token)] += 1

next_day_matrix = np.column_stack((
    np.eye(STATE_COUNT, dtype=int)[:, STATE_COUNT - 1:] + np.eye(STATE_COUNT, dtype=int)[:, 6:7],
    np.eye(STATE_COUNT, dtype=int)[:, :STATE_COUNT - 1]))

def fish_count(duration_days):
    return np.sum(np.linalg.matrix_power(next_day_matrix, duration_days).dot(initial))

print("part1={}".format(fish_count(80)))
print("part2={}".format(fish_count(256)))
