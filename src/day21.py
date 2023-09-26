def solve_part1():
    player_count = 2
    player_positions = [1, 4]
    player_scores = [0, 0]
    rolls = 0
    turn = 0
    max_score = 1000

    while max(player_scores) < max_score:
        roll1 = rolls % 100 + 1
        roll2 = (rolls + 1) % 100 + 1
        roll3 = (rolls + 2) % 100 + 1
        roll = roll1 + roll2 + roll3
        rolls += 3
        player_positions[turn] = (player_positions[turn] + roll) % 10
        player_scores[turn] += player_positions[turn] + 1
        turn = (turn + 1) % player_count

    return min(player_scores) * rolls

def solve_part2():
    player_count = 2
    player_positions = [1, 4]
    max_score = 21
    cache = {}

    def find_wins(pos1, pos2, score1, score2, turn):
        if score1 >= max_score:
            return 1, 0
        if score2 >= max_score:
            return 0, 1
        
        win1 = 0
        win2 = 0
        for roll, frequency in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]:
            if turn == 0:
                pos = (pos1 + roll) % 10
                subwin1, subwin2 = find_wins_cached(pos, pos2, score1 + pos + 1, score2, 1)
            else:
                pos = (pos2 + roll) % 10
                subwin1, subwin2 = find_wins_cached(pos1, pos, score1, score2 + pos + 1, 0)
            win1 += subwin1 * frequency
            win2 += subwin2 * frequency
        return win1, win2

    def find_wins_cached(pos1, pos2, score1, score2, turn):
        solution = cache.get((pos1, pos2, score1, score2, turn))
        if not solution is None:
            return solution
        solution = tuple(find_wins(pos1, pos2, score1, score2, turn))
        cache[(pos1, pos2, score1, score2, turn)] = solution
        return solution

    win1, win2 = find_wins(1, 4, 0, 0, 0)
    return max(win1, win2)

print('part1', solve_part1())
print('part2', solve_part2())
