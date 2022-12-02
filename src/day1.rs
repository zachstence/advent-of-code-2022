use std::cmp;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    // Append newline so last elf gets read properly
    let input = String::from(input) + "\n";
    
    let tokens = input.split('\n');

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

    max_calories
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    // Append newline so last elf gets read properly
    let input = String::from(input) + "\n";
    
    let tokens: Vec<&str> = input.split('\n').collect();

    sum_of_top_calories(&tokens, 3)
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
    #[allow(clippy::needless_range_loop)]
    for i in 0..top {
        sum += calories[i];
    }

    sum
}