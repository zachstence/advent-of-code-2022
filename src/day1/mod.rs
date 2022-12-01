use std::fs;
use std::cmp;

pub fn part1(input_file: &str) -> String {
    // Append newline so last elf gets read properly
    let input = fs::read_to_string(input_file).unwrap() + "\n";
    
    let tokens = input.split("\n");

    let mut max_calories = 0;
    let mut calories = 0;

    for token in tokens {
        // Empty line means we're moving to the next elf, so save current calories as max
        // Non-empty line means we're adding calories
        if token.is_empty() {
            max_calories = cmp::max(max_calories, calories);
            calories = 0;
        } else {
            let c = token.parse::<i32>().unwrap();
            calories += c;
        }
    }

    return max_calories.to_string();
}

pub fn part2(input_file: &str) -> String {
    // Append newline so last elf gets read properly
    let input = fs::read_to_string(input_file).unwrap() + "\n";
    
    let tokens: Vec<&str> = input.split("\n").collect();
    return sum_of_top_calories(&tokens, 3).to_string();
}

fn sum_of_top_calories(tokens: &Vec<&str>, top: usize) -> i32 {
    let mut calories = Vec::new();
    let mut tmp = 0;

    for token in tokens {
        // Empty line means we're moving to the next elf, so save current calories
        // Non-empty line means we're adding calories
        if token.is_empty() {
            calories.push(tmp);
            tmp = 0;
        } else {
            let c = token.parse::<i32>().unwrap();
            tmp += c;
        }
    }

    calories.sort_by(|a, b| b.cmp(a));

    let mut sum = 0;
    for i in 0..top {
        sum += calories[i];
    }

    return sum;
}