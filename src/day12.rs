use std::collections::BTreeSet;
use std::collections::BTreeMap;
type PuzzleInput = BTreeMap<String, Vec<String>>;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data).expect("Unable to parse file");
    println!("part1={}", solve_part1(&puzzle_input));
    println!("part2={}", solve_part2(&mut puzzle_input.clone()));
}

fn parse_puzzle_input(file_data: String) -> Option<PuzzleInput> {
    let mut puzzle_input = BTreeMap::new();

    fn add_edge(puzzle_input: &mut PuzzleInput, start: &String, end: &String) {
        if end == "start" || start == "end" {
            return;
        }

        puzzle_input.entry(start.clone()).or_insert(vec![]).push(end.clone());
    }

    for line in file_data.split("\n") {
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<String> = line.split("-").map(String::from).collect();
        if tokens.len() != 2 {
            return None;
        }

        add_edge(&mut puzzle_input, &tokens[0], &tokens[1]);
        add_edge(&mut puzzle_input, &tokens[1], &tokens[0]);
    }

    Some(puzzle_input)
}

fn solve_part1(puzzle_input: &PuzzleInput) -> i32 {
    let visited = BTreeSet::new();
    traverse(puzzle_input, &visited, &String::from("start"), &Some(String::from("")))
}

fn solve_part2(puzzle_input: &PuzzleInput) -> i32 {
    let visited = BTreeSet::new();
    traverse(puzzle_input, &visited, &String::from("start"), &None)
}

fn traverse(
        puzzle_input: &PuzzleInput,
        curr_visited: &BTreeSet<String>,
        curr_node: &String,
        curr_double_visited: &Option<String>) -> i32 {
    if curr_node == "end" {
        return 1;
    }

    if !is_big_cave(curr_node) && curr_visited.contains(curr_node) && curr_double_visited.is_some() {
        return 0;
    }

    let mut next_visited = curr_visited.clone();
    let mut next_double_visited = curr_double_visited.clone();
    if !is_big_cave(curr_node) {
        if curr_visited.contains(curr_node) {
            next_double_visited = Some(curr_node.clone());
        } else {
            next_visited.insert(curr_node.clone());
        }
    }

    let mut total = 0;

    for next_node in &puzzle_input[curr_node] {
        total += traverse(puzzle_input, &next_visited, &next_node, &next_double_visited);
    }

    total
}

fn is_big_cave(node: &String) -> bool {
    &node.to_uppercase() == node
}
