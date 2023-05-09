use std::collections::HashMap;

type PuzzleInput = Vec<Line>;

#[derive(Clone, Debug)]
struct Line {
    start: Point,
    end: Point
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32
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
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line_data: &str) -> Line {
    let tokens: Vec<&str> = line_data.split(" -> ").collect();
    Line {
        start: parse_point(tokens[0]),
        end: parse_point(tokens[1])
    }
}

fn parse_point(point_data: &str) -> Point {
    let tokens: Vec<&str> = point_data.split(",").collect();
    Point {
        x: tokens[0].parse().unwrap(),
        y: tokens[1].parse().unwrap()
    }
}

fn solve_part1(puzzle_input: &PuzzleInput) -> usize {
    solve_part2(&puzzle_input
        .iter()
        .filter(|line| line.start.x == line.end.x || line.start.y == line.end.y)
        .cloned()
        .collect())
}

fn solve_part2(puzzle_input: &PuzzleInput) -> usize {
    let mut counters: HashMap<Point, u32> = HashMap::new();

    for line in puzzle_input.iter() {
        let mut curr = line.start.clone();
        update_counters(&curr, &mut counters);
        while curr != line.end {
            curr.x += (line.end.x - curr.x).signum();
            curr.y += (line.end.y - curr.y).signum();
            update_counters(&curr, &mut counters);
        }
    }

    counters
        .values()
        .filter(|&count| count >= &2)
        .count()
}

fn update_counters(point: &Point, counters: &mut HashMap<Point, u32>) {
    counters.insert(point.clone(), *counters.get(point).unwrap_or(&0) + 1);
}