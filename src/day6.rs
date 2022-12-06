use std::collections::HashSet;
use std::collections::LinkedList;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_start(input, 4)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_start(input, 14)
}

fn find_start(input: &str, x: usize) -> usize {
    let mut iter = input.chars();
    let mut q = iter.by_ref().take(x).collect::<LinkedList<char>>();

    for (i, ch) in iter.enumerate() {
        if has_x_unique(&q, x) {
            return i + x;
        }
        q.pop_front();
        q.push_back(ch);
    }

    panic!("Didn't find a start sequence");
}

fn has_x_unique(l: &LinkedList<char>, x: usize) -> bool {
    l.iter().collect::<HashSet<_>>().len() == x
}
