use std::collections::BTreeSet;
use std::iter::FromIterator;

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Debug)]
struct Point {
    x: u32,
    y: u32
}

#[derive(Clone, Debug)]
struct Fold {
    axis: char,
    dist: u32
}

#[derive(Debug)]
struct PuzzleInput {
    points: BTreeSet<Point>,
    folds: Vec<Fold>
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data);
    println!("part1={}", solve_part1(&puzzle_input));
    solve_part2(&puzzle_input);
}

fn parse_puzzle_input(file_data: String) -> PuzzleInput {
    let segments: Vec<&str> = file_data.split("\n\n").collect();
    let points = parse_points(segments.get(0).expect("No points"));
    let folds = parse_folds(segments.get(1).expect("No folds"));

    PuzzleInput {
        points: points,
        folds: folds
    }
}

fn parse_points(points_data: &str) -> BTreeSet<Point> {
    let points: Vec<Point> = points_data
        .lines()
        .filter(is_nonempty_line)
        .map(parse_point)
        .collect();
    BTreeSet::from_iter(points.iter().cloned())
}

fn parse_point(point_data: &str) -> Point {
    let numbers: Vec<&str> = point_data.split(",").collect();
    Point { x: numbers.get(0).expect("Line does not contain x").parse().expect("x is not an integer"),
        y: numbers.get(1).expect("Line does not contain y").parse().expect("y is not an integer")}
}

fn parse_folds(folds_data: &str) -> Vec<Fold> {
    folds_data.lines()
        .filter(is_nonempty_line)
        .map(parse_fold)
        .collect()
}

fn parse_fold(fold_data: &str) -> Fold {
    let line_data = fold_data.split(" ").last().expect("Fold does not contain a word");
    let mut tokens = line_data.split("=");

    Fold {
        axis: tokens.next().expect("No expression in line").chars().next().expect("axis is not a char"),
        dist: tokens.last().expect("No expression in line").parse().expect("dist is not an integer")
    }
}

fn is_nonempty_line(line: &&str) -> bool {
    !line.is_empty()
}

fn solve_part1(puzzle_input: &PuzzleInput) -> usize {
    let mut folded_points: BTreeSet<Point> = BTreeSet::new();
    for point in &puzzle_input.points {
        folded_points.insert(fold_point(&point, puzzle_input.folds.first().expect("Fold expected")));
    }
    folded_points.len()
}

fn solve_part2(puzzle_input: &PuzzleInput) {
    render(apply_folds(puzzle_input));
}

fn apply_folds(puzzle_input: &PuzzleInput) -> BTreeSet<Point> {
    let mut points = puzzle_input.points.clone();
    for fold in &puzzle_input.folds {
        let mut folded_points: BTreeSet<Point> = BTreeSet::new();
        for point in &points {
            folded_points.insert(fold_point(&point, &fold));
        }
        points = folded_points;
    }
    points
}

fn render(points: BTreeSet<Point>) {
    let max_x = points.iter().map(|point| point.x).max().expect("A point was expected");
    let max_y = points.iter().map(|point| point.y).max().expect("A point was expected");
    for y in 0..=max_y {
        for x in 0..=max_x {
            if points.contains(&Point { x: x, y: y }) {
                print!("##");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn fold_point(point: &Point, fold: &Fold) -> Point {
    match fold {
       Fold { axis: 'x', dist } => Point { x: reflect(point.x, *dist), y: point.y },
       Fold { axis: 'y', dist } => Point { x: point.x, y: reflect(point.y, *dist) },
       _ => point.clone()
    }
}

fn reflect(x: u32, z: u32) -> u32 {
    if x > z {
        2 * z - x
    } else {
        x
    }
}