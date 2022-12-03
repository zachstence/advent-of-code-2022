use std::collections::HashSet;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let (first, second) = chars.split_at(chars.len() / 2);
            let items = first.iter().collect::<HashSet<&char>>();
            
            let duplicate = second.iter().find(|ch| items.contains(ch)).unwrap();
            get_priority(duplicate)
        })
        .sum::<u32>()
}

fn get_priority(c: &char) -> u32 {
    if *c >= 'a' && *c <= 'z' {
        (*c as u32) - 97 + 1
    } else if *c >= 'A' && *c <= 'Z' {
        (*c as u32) - 65 + 27
    } else {
        panic!("Non-alphabetical character '{}'", *c)
    }
}
