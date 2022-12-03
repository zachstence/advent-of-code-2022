use std::collections::HashSet;
use itertools::Itertools;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let (first, second) = chars.split_at(chars.len() / 2);
            let items = first.iter().collect::<HashSet<&char>>();
            
            let duplicate = second.iter().find(|ch| items.contains(ch)).unwrap();
            get_priority(*duplicate)
        })
        .sum::<u32>()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(l1, l2, l3)| {
            let set1 = l1.chars().collect::<HashSet<char>>();
            let set2 = l2.chars().collect::<HashSet<char>>();

            let triplicate = l3.chars().find(|ch| set1.contains(ch) && set2.contains(ch)).unwrap();
            get_priority(triplicate)
        })
        .sum()
}

fn get_priority(c: char) -> u32 {
    if ('a'..='z').contains(&c) {
        (c as u32) - 97 + 1
    } else if ('A'..='Z').contains(&c) {
        (c as u32) - 65 + 27
    } else {
        panic!("Non-alphabetical character '{}'", c)
    }
}
