use std::collections::HashMap;

type Point = (usize, usize);
type Grid = HashMap<i32, Point>;

#[derive(Debug)]
struct PuzzleInput {
    sequence: Vec<i32>,
    grids: Vec<Grid>,
    grid_size: usize
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data);
    println!("part1={:?}", solve_part1(&puzzle_input));
    println!("part2={:?}", solve_part2(&puzzle_input));
}

fn parse_puzzle_input(file_data: String) -> PuzzleInput {
    let groups: Vec<&str> = file_data.trim().split("\n\n").collect();
    let sequence: Vec<i32> = groups[0]
        .split(",")
        .map(|token| token.parse().expect("Unable to parse as integer"))
        .collect();
    let grids: Vec<Grid> = groups.iter().skip(1).map(|group| parse_grid(group)).collect();
    let grid_size: usize = groups[1].split("\n").collect::<Vec<_>>().len();

    PuzzleInput {
        sequence: sequence,
        grids: grids,
        grid_size: grid_size
    }
}

fn parse_grid(grid_data: &str) -> Grid {
    let mut result: Grid = HashMap::new();

    for (row, row_data) in grid_data.split("\n").enumerate() {
        let cell_datas = row_data.split(" ").filter(|&cell_data| !cell_data.is_empty());
        for (column, cell_data) in cell_datas.enumerate() {
            let cell = cell_data.parse().expect("Unable to parse as integer");
            result.insert(cell, (column, row));
        }
    }
    
    result
}

#[derive(Debug)]
struct GridVisit {
    rows: Vec<u32>,
    cols: Vec<u32>,
    total: i32,
    complete: bool
}

fn solve_part1(puzzle_input: &PuzzleInput) -> Option<i32> {
    find_solutions(puzzle_input).first().copied()
}

fn solve_part2(puzzle_input: &PuzzleInput) -> Option<i32> {
    find_solutions(puzzle_input).last().copied()
}

fn find_solutions(puzzle_input: &PuzzleInput) -> Vec<i32> {
    let mut visits: Vec<GridVisit> = puzzle_input.grids
        .iter()
        .map(|_| GridVisit {
            rows: vec![0; puzzle_input.grid_size],
            cols: vec![0; puzzle_input.grid_size],
            total: 0,
            complete: false})
        .collect();
    let mut solutions: Vec<i32> = vec![];

    for number in &puzzle_input.sequence {
        for (index, grid) in puzzle_input.grids.iter().enumerate() {
            if let Some((col, row)) = grid.get(&number) {
                let visit = &mut visits[index];
                visit.cols[*col] += 1;
                visit.rows[*row] += 1;
                visit.total += number;

                if !visit.complete && is_visit_complete(puzzle_input.grid_size, visit) {
                    visit.complete = true;
                    solutions.push((grid_total(grid) - visits[index].total) * number);
                }
            }
        }
    }

    solutions
}

fn is_visit_complete(grid_size: usize, visit: &GridVisit) -> bool {
    let size: u32 = grid_size as u32;
    visit.rows.iter().any(|row| row == &size) || visit.cols.iter().any(|col| col == &size)
}

fn grid_total(grid: &Grid) -> i32 {
    grid.keys().sum()
}