use std::fs;
use std::cmp;

pub fn part1(input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).unwrap();
    
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

    return max_calories;
}
