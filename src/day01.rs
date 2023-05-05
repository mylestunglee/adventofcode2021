fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let depths: Vec<i32> = file_data.split('\n')
        .filter(|&line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    println!("part1={}", count_increasing(&depths));
    println!("part1={}", count_increasing(&moving_sum(&depths)));
}

fn count_increasing(numbers: &Vec<i32>) -> i32 {
    let mut count: i32 = 0;

    for (index, _) in numbers.iter().enumerate().skip(1) {
        if numbers[index - 1] < numbers[index] {
            count += 1;
        }
    }

    count
}

fn moving_sum(numbers: &Vec<i32>) -> Vec<i32> {
    let mut windows: Vec<i32> = vec![];

    for (index, _) in numbers.iter().enumerate().skip(2) {
        windows.push(numbers[index - 2] + numbers[index - 1] + numbers[index])
    }

    windows
}
