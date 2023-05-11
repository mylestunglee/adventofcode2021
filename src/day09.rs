use std::collections::BTreeSet;

type Point = (usize, usize);
type Grid = Vec<Vec<u32>>;

#[derive(Debug)]
struct PuzzleInput {
    grid: Grid,
    width: usize,
    height: usize
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data).expect("Unable to parse file");
    println!("part1={}", solve_part1(&puzzle_input));
    println!("part2={}", solve_part2(&puzzle_input));
}

fn parse_puzzle_input(file_data: String) -> Option<PuzzleInput> {
    let grid: Option<Grid> = file_data
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();
    let width: Option<usize> = grid.as_ref().and_then(|grid| grid.first().cloned()).map(|row| row.len());
    let height: Option<usize> = grid.as_ref().map(|grid| grid.len());

    match (grid, width, height) {
        (Some(grid), Some(width), Some(height)) => Some(PuzzleInput {
            grid: grid,
            width: width,
            height: height
        }),
        _ => None
    }
}

fn solve_part1(puzzle_input: &PuzzleInput) -> u32 {
    (0..puzzle_input.width).map(|x| (0..puzzle_input.height).map(|y| risk_level(&puzzle_input.grid, x, y)).sum::<u32>()).sum()
}

fn is_low_point(grid: &Grid, x: usize, y: usize) -> bool {
    let adjacents = [
        get_at(grid, x.checked_sub(1), Some(y)),
        get_at(grid, x.checked_add(1), Some(y)),
        get_at(grid, Some(x), y.checked_sub(1)),
        get_at(grid, Some(x), y.checked_add(1))];
    let sample: &u32 = get_at(grid, Some(x), Some(y)).expect("Sample must be in grid");

    adjacents.iter().all(|adjacent| adjacent.map(|adjacent| sample < adjacent).unwrap_or(true))
}

fn risk_level(grid: &Grid, x: usize, y: usize) -> u32 {
    let sample: &u32 = get_at(grid, Some(x), Some(y)).expect("Sample must be in grid");

    if is_low_point(grid, x, y) {
        sample + 1
    } else {
        0
    }
}

fn get_at(grid: &Grid, x: Option<usize>, y: Option<usize>) -> Option<&u32> {
    match (x, y) {
        (Some(x), Some(y)) => grid.get(y).and_then(|row| row.get(x)),
        _ => None
    }
}

fn solve_part2(puzzle_input: &PuzzleInput) -> usize {
    let low_points: Vec<Point> = (0..puzzle_input.width)
        .map(|x| (0..puzzle_input.height)
            .map(|y| {
                if is_low_point(&puzzle_input.grid, x, y) {
                    Some((x, y))
                } else {
                    None
                }
            }).flatten().collect::<Vec<_>>()).flatten().collect();
    let mut lengths: Vec<usize> = low_points
        .iter()
        .map(|low_point| fill_basin(&puzzle_input.grid, low_point))
        .collect::<BTreeSet<BTreeSet<Point>>>() // Two basins may share a low point
        .iter()
        .map(BTreeSet::len)
        .collect();
    lengths.sort();
    lengths.iter().rev().take(3).product()
}

fn fill_basin(grid: &Grid, point: &Point) -> BTreeSet<Point> {
    let mut basin: BTreeSet<Point> = BTreeSet::new();
    fill_basin_recursive(grid, *point, &mut basin);
    basin
}

fn fill_basin_recursive(grid: &Grid, (x, y): Point, visited: &mut BTreeSet<Point>) {
    if visited.contains(&(x, y)) {
        return;
    }

    visited.insert((x, y));

    let adjacents = [
        (x.checked_sub(1), Some(y)),
        (x.checked_add(1), Some(y)),
        (Some(x), y.checked_sub(1)),
        (Some(x), y.checked_add(1))];

    for (x, y) in adjacents {
        match get_at(grid, x, y) {
            Some(9) => {}
            Some(_) => {
                fill_basin_recursive(grid, (x.unwrap(), y.unwrap()), visited);
            }
            None => {}
        }
    }
}
