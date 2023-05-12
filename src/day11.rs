type Point = (usize, usize);
type Grid = Vec<Vec<i32>>;

const MAX_ENERGY: i32 = 9;
const FLASHED_ENERGY: i32 = -1;

#[derive(Debug, Clone)]
struct PuzzleInput {
    grid: Grid,
    width: usize,
    height: usize
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: PuzzleInput = parse_puzzle_input(file_data).expect("Unable to parse file");
    println!("part1={}", solve_part1(&mut puzzle_input.clone()));
    println!("part2={}", solve_part2(&mut puzzle_input.clone()));
}

fn parse_puzzle_input(file_data: String) -> Option<PuzzleInput> {
    let grid: Option<Grid> = file_data
        .trim()
        .split("\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).map(|x| x as i32)).collect())
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

fn solve_part1(puzzle_input: &mut PuzzleInput) -> u32 {
    let mut total_flashed = 0;
    for _ in 0..100 {
        let jellyfishes = find_energized(puzzle_input);
        total_flashed += flash_energized(puzzle_input, jellyfishes);
        increment_energies(puzzle_input);
    }
    total_flashed
}

fn solve_part2(puzzle_input: &mut PuzzleInput) -> u32 {
    let mut time = 0;
    loop {
        time += 1;
        let jellyfishes = find_energized(puzzle_input);
        if flash_energized(puzzle_input, jellyfishes) == (puzzle_input.width * puzzle_input.height) as u32 {
            return time;
        }
        increment_energies(puzzle_input);
    }
}

fn find_energized(puzzle_input: &mut PuzzleInput) -> Vec<Point> {
    (0..puzzle_input.width)
        .map(|x| (0..puzzle_input.height)
            .map(|y| (x, y))
            .filter(|(x, y)| puzzle_input.grid[*y][*x] >= MAX_ENERGY)
            .collect::<Vec<_>>())
        .flatten()
        .collect()
}

fn flash_energized(puzzle_input: &mut PuzzleInput, jellyfishes: Vec<Point>) -> u32 {
    let mut total_flashed = 0;
    for jellyfish in jellyfishes {
        total_flashed += flash_energized_recursive(puzzle_input, jellyfish);
    }
    total_flashed
}

fn flash_energized_recursive(puzzle_input: &mut PuzzleInput, (x, y): Point) -> u32 {
    if puzzle_input.grid[y][x] == FLASHED_ENERGY {
        return 0;
    }

    puzzle_input.grid[y][x] = FLASHED_ENERGY;
    let mut total_flashed = 1;

    for offset_x in -1_i32..=1 {
        for offset_y in -1_i32..=1 {
            let grid_x = x as i32 + offset_x;
            let grid_y = y as i32 + offset_y;
            if 0 <= grid_x
                && grid_x < puzzle_input.width as i32
                && 0 <= grid_y
                && grid_y < puzzle_input.height as i32
                && !(offset_x == 0 && offset_y == 0) {
                
                if puzzle_input.grid[grid_y as usize][grid_x as usize] != FLASHED_ENERGY {
                    puzzle_input.grid[grid_y as usize][grid_x as usize] += 1;
                    if puzzle_input.grid[grid_y as usize][grid_x as usize] >= MAX_ENERGY {
                        total_flashed += flash_energized_recursive(puzzle_input, (grid_x as usize, grid_y as usize));
                    }
                }
            }
        }
    }

    total_flashed
}

fn increment_energies(puzzle_input: &mut PuzzleInput) {
    puzzle_input.grid
        .iter_mut()
        .for_each(|row| row
            .iter_mut()
            .for_each(|energy| { *energy += 1; }));
}
