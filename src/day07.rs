fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let puzzle_input: Vec<i32> = parse_puzzle_input(&file_data);
    println!("part1={}", solve_part1(&puzzle_input));
    println!("part2={}", solve_part2(&puzzle_input));
}

fn parse_puzzle_input(file_data: &String) -> Vec<i32> {
    file_data
        .trim()
        .split(",")
        .map(|token| token.parse()
        .expect("Unable to convert to i32"))
        .collect()
}

fn solve_part1(puzzle_input: &Vec<i32>) -> i32 {
    let min: i32 = *puzzle_input.iter().min().unwrap();
    let max: i32 = *puzzle_input.iter().max().unwrap();
    let mut fuels: Vec<i32> = vec![];

    for middle in min..=max {
        fuels.push(calc_fuel(puzzle_input, &middle));
    }

    return *fuels.iter().min().unwrap()
}

fn calc_fuel(puzzle_input: &Vec<i32>, middle: &i32) -> i32 {
    puzzle_input.iter().map(|x| (x - middle).abs()).sum::<i32>()
}

fn solve_part2(puzzle_input: &Vec<i32>) -> i32 {
    let min: i32 = *puzzle_input.iter().min().unwrap();
    let max: i32 = *puzzle_input.iter().max().unwrap();
    let mut fuels: Vec<i32> = vec![];

    for middle in min..=max {
        fuels.push(calc_fuel_expensive(puzzle_input, &middle));
    }

    return *fuels.iter().min().unwrap()
}

fn calc_fuel_expensive(puzzle_input: &Vec<i32>, middle: &i32) -> i32 {
    puzzle_input.iter().map(|x| triangle_number((x - middle).abs())).sum::<i32>()
}

fn triangle_number(x: i32) -> i32 {
    (x * (x + 1)) / 2
}
