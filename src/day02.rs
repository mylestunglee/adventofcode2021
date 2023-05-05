struct Instruction {
    direction: String,
    distance: i32
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let instructions: Vec<Instruction> = file_data.split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            Instruction {direction: String::from(tokens[0]), distance: tokens[1].parse().unwrap()}
        })
        .collect();
    
    println!("part1={}", solve_part1(&instructions));
    println!("part2={}", solve_part2(&instructions));
}

fn solve_part1(instructions: &Vec<Instruction>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for instruction in instructions {
        if instruction.direction == "forward" {
            x += instruction.distance;
        } else if instruction.direction == "up" {
            y -= instruction.distance;
        } else if instruction.direction == "down" {
            y += instruction.distance;
        } else {
            println!("invalid instruction");
        }
    }

    x * y
}

fn solve_part2(instructions: &Vec<Instruction>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut aim: i32 = 0;

    for instruction in instructions {
        if instruction.direction == "forward" {
            x += instruction.distance;
            y += aim * instruction.distance;
        } else if instruction.direction == "up" {
            aim -= instruction.distance;
        } else if instruction.direction == "down" {
            aim += instruction.distance;
        } else {
            println!("invalid instruction");
        }
    }

    x * y
}
