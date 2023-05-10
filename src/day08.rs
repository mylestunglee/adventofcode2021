use std::collections::HashSet;
use std::iter::FromIterator;

type Pattern = HashSet<char>;
type Patterns = Vec<Pattern>;
type PuzzleInput = Vec<Dataset>;

#[derive(Debug)]
struct Dataset {
    training: Patterns,
    testing: Patterns
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data);
    println!("part1={}", solve_part1(&puzzle_input));
    println!("part2={}", solve_part2(&puzzle_input));
}

fn parse_puzzle_input(file_data: String) -> PuzzleInput {
    file_data
        .trim()
        .split("\n")
        .map(String::from)
        .map(parse_dataset)
        .collect()
}

fn parse_dataset(dataset_data: String) -> Dataset {
    let tokens: Vec<String> = dataset_data
        .split(" | ")
        .map(String::from)
        .collect();
    Dataset {
        training: parse_patterns(tokens[0].clone()),
        testing: parse_patterns(tokens[1].clone())
    }
}

fn parse_patterns(patterns_data: String) -> Patterns {
    patterns_data
        .split(" ")
        .map(String::from)
        .map(parse_pattern)
        .collect()
}

fn parse_pattern(pattern_data: String) -> Pattern {
    HashSet::from_iter(pattern_data.chars())
}

fn solve_part1(puzzle_input: &PuzzleInput) -> usize {
    puzzle_input
        .iter()
        .map(|dataset| dataset.testing
            .iter()
            .filter(|pattern| {
                let length = pattern.len();
                length == 2 || length == 4 || length == 3 || length == 7
            })
            .count())
        .sum()
}

fn solve_part2(puzzle_input: &PuzzleInput) -> usize {
    puzzle_input.iter().map(|dataset| solve_dataset(dataset)).sum()
}

fn solve_dataset(dataset: &Dataset) -> usize {
    let encoding = deduce_encoding(dataset);
    let digits: Vec<usize> = dataset.testing
        .iter()
        .map(|encoded| encoding.iter().position(|pattern| encoded == pattern).unwrap())
        .collect();

    let mut solution = 0;
    for digit in digits {
        solution = solution * 10 + digit;
    }

    solution
}

fn deduce_encoding(dataset: &Dataset) -> Patterns {
    let display_segments: [HashSet<usize>; 10] = [
        HashSet::from([0, 1, 2, 4, 5, 6]),
        HashSet::from([2, 5]),
        HashSet::from([0, 2, 3, 4, 6]),
        HashSet::from([0, 2, 3, 5, 6]),
        HashSet::from([1, 2, 3, 5]),
        HashSet::from([0, 1, 3, 5, 6]),
        HashSet::from([0, 1, 3, 4, 5, 6]),
        HashSet::from([0, 2, 5]),
        HashSet::from([0, 1, 2, 3, 4, 5, 6]),
        HashSet::from([0, 1, 2, 3, 5, 6])
    ];

    // determine encoding
    let mut possible_segments: [Pattern; 7] = Default::default();
    for i in 0..7 {
        possible_segments[i] = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    }

    // deduce unique-lengthed digts
    for digit in [4, 7] {
        let possible_chars = dataset.training
            .iter()
            .filter(|pattern| pattern.len() == display_segments[digit].len())
            .next()
            .unwrap();
        for display_segment in 0_usize..7 {
            if display_segments[digit].contains(&display_segment) {
                possible_segments[display_segment] = &possible_segments[display_segment] & possible_chars;
            } else {
                possible_segments[display_segment] = &possible_segments[display_segment] - possible_chars;
            }
        }
    }

    // deduce encoding because 3 is a superset of segments of 7
    {
        let digit = 3;
        let seven_chars = dataset.training
            .iter()
            .filter(|pattern| pattern.len() == display_segments[7].len())
            .next()
            .unwrap();
        let three_chars = dataset.training
            .iter()
            .filter(|pattern| pattern.len() == display_segments[digit].len() && pattern.is_superset(seven_chars))
            .next()
            .unwrap();
        for display_segment in 0_usize..7 {
            if display_segments[digit].contains(&display_segment) {
                possible_segments[display_segment] = &possible_segments[display_segment] & three_chars;
            } else {
                possible_segments[display_segment] = &possible_segments[display_segment] - three_chars;
            }
        }
    }

    // deduce encoding for segments 2 and 5
    {
        let digit = 2;
        let two_chars = dataset.training.iter().filter(|pattern|
                pattern.contains(possible_segments[0].iter().next().unwrap())
            && !pattern.contains(possible_segments[1].iter().next().unwrap())
            &&  pattern.contains(possible_segments[3].iter().next().unwrap())
            &&  pattern.contains(possible_segments[4].iter().next().unwrap())
            &&  pattern.contains(possible_segments[6].iter().next().unwrap())).next().unwrap();

        for display_segment in 0_usize..7 {
            if display_segments[digit].contains(&display_segment) {
                possible_segments[display_segment] = &possible_segments[display_segment] & &two_chars;
            } else {
                possible_segments[display_segment] = &possible_segments[display_segment] - &two_chars;
            }
        }
    }

    (0..10).map(|digit| {
        display_segments[digit]
            .iter()
            .map(|segment| possible_segments[*segment].iter().cloned().next().unwrap())
            .collect()
    }).collect()
}
