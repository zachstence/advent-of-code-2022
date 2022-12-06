use std::collections::HashSet;
use std::collections::LinkedList;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut iter = input.chars();
    let mut q = iter.by_ref().take(4).collect::<LinkedList<char>>();

    for (i, ch) in iter.enumerate() {
        if has_four_unique(&q) {
            return i + 4;
        }
        q.pop_front();
        q.push_back(ch);
    }

    panic!("Didn't find a start sequence");
}

fn has_four_unique(v: &LinkedList<char>) -> bool {
    v.iter().collect::<HashSet<_>>().len() == 4
}
