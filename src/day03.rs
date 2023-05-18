fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_data = std::fs::read_to_string(&args[1]).expect("Unable to read file");
    let lines: Vec<String> = file_data.split('\n')
        .filter(|&line| !line.is_empty())
        .map(String::from)
        .collect();
    println!("part1={}", solve_part1(&lines));
    println!("part2={}", solve_part2(&lines));
}

fn solve_part1(lines: &Vec<String>) -> i32 {
    let length: usize = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap();
    let mut counts: Vec<i32> = vec![0; length];
    for line in lines {
        for (column, digit) in line.chars().enumerate() {
            if digit == '0' {
                counts[column] -= 1;
            } else {
                counts[column] += 1;
            }
        }
    }
    let negative_counts: Vec<i32> = counts.iter().map(|count| -count).collect();

    counts_as_binary(&counts) * counts_as_binary(&negative_counts)
}

fn counts_as_binary(counts: &Vec<i32>) -> i32 {
    counts.iter().rev().enumerate().map(|(index, count)| {
        let sign = if *count > 0 {1} else {0};
        let scalar = 2_i32.pow(index as u32);
        sign * scalar
    }).sum()
}

fn solve_part2(lines: &Vec<String>) -> i32 {
    extract_value_bitwise(lines, |count| count >= 0) * extract_value_bitwise(lines, |count| count < 0)
}

fn extract_value_bitwise(lines: &Vec<String>, count_predicate: fn(i32) -> bool) -> i32 {
    let length: usize = lines
        .iter()
        .map(|line| line.len())
        .max()
        .unwrap();
    let mut filtered_lines = lines.clone();
    for column in 0..length {
        if filtered_lines.len() <= 1 {
            break;
        }
        let count = count_ones_at_column(&filtered_lines, column);
        if count_predicate(count) {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.chars().nth(column).unwrap() == '1').collect();
        } else {
            filtered_lines = filtered_lines.into_iter().filter(|line| line.chars().nth(column).unwrap() == '0').collect();
        }
    }

    i32::from_str_radix(filtered_lines[0].as_str(), 2).unwrap()
}

fn count_ones_at_column(lines: &Vec<String>, column: usize) -> i32 {
    let mut count = 0;
    for line in lines {
        if line.chars().nth(column).unwrap() == '0' {
            count -= 1;
        } else {
            count += 1;
        }
    }

    count
}
