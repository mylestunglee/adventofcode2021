import sys
from itertools import pairwise
from collections import Counter

with open(sys.argv[1]) as file:
    data = file.read()

def parse_data(data):
    template, rules_data = data.strip().split('\n\n')
    template_start = template[0]
    template_pairs = Counter(pairwise(template))
    rules = {(line[0], line[1]): line[-1] for line in rules_data.splitlines()}
    return template_start, template_pairs, rules

def grow_template(template_pairs, rules):
    result = Counter()
    for (start_char, end_char), count in template_pairs.items():
        middle_char = rules.get((start_char, end_char))
        if not middle_char is None:
            result[(start_char, middle_char)] += count
            result[(middle_char, end_char)] += count
    return result

def count_letters(template_start, template_pairs):
    counter = Counter({template_start: 1})
    for (_, end_char), count in template_pairs.items():
        counter[end_char] += count
    return counter

def score_letters(counter):
    sorted_letters = counter.most_common()
    return sorted_letters[0][1] - sorted_letters[-1][1]

def solve(iterations):
    template_start, template_pairs, rules = parse_data(data)
    for _ in range(iterations):
        template_pairs = grow_template(template_pairs, rules)
    return score_letters(count_letters(template_start, template_pairs))


print('part1={}'.format(solve(10)))
print('part2={}'.format(solve(40)))
