use std::collections::BTreeMap;
type PuzzleInput = Vec<String>;

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
        .collect()
}

fn solve_part1(puzzle_input: &PuzzleInput) -> i32 {
    puzzle_input
        .iter()
        .map(syntax_error_score)
        .sum()
}

fn syntax_error_score(line: &String) -> i32 {
    let chunk_symbols = BTreeMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score = BTreeMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut opened_chunks: Vec<char> = vec![];
    for symbol in line.chars() {
        if chunk_symbols.contains_key(&symbol) {
            opened_chunks.push(symbol);
        } else if opened_chunks.is_empty() {
            // incomplete
            break;
        } else if chunk_symbols[opened_chunks.last().unwrap()] == symbol {
            opened_chunks.truncate(opened_chunks.len() - 1);        
        } else {
            return score[&symbol];
        }
    }
    0
}

fn solve_part2(puzzle_input: &PuzzleInput) -> i64 {
    let mut scores: Vec<i64> = puzzle_input
        .iter()
        .map(autocomplete_score)
        .filter(|score| score > &0)
        .collect();
    scores.sort();
    scores[(scores.len() - 1) / 2]
}

fn autocomplete_score(line: &String) -> i64 {
    let chunk_symbols = BTreeMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score = BTreeMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut opened_chunks: Vec<char> = vec![];
    for symbol in line.chars() {
        if chunk_symbols.contains_key(&symbol) {
            opened_chunks.push(symbol);
        } else if opened_chunks.is_empty() {
            // incomplete
            break;
        } else if chunk_symbols[opened_chunks.last().unwrap()] == symbol {
            opened_chunks.truncate(opened_chunks.len() - 1);        
        } else {
            // syntax error
            return 0;
        }
    }
    opened_chunks
        .iter()
        .rev()
        .map(|symbol| score[symbol])
        .fold(0, |acc, score| acc * 5 + score)
}
