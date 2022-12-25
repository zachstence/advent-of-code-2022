use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

const SAND_DROP: Point = (500, 0);

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut rocks: HashSet<Object> = HashSet::new();

    input
        .lines()
        .for_each(|line| {
            line
                .split(" -> ")
                .map(|token| token.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .tuple_windows()
                .for_each(|((start_x, start_y), (end_x, end_y))| {
                    println!("({start_x},{start_y}), ({end_x},{end_y})");
                    if start_y == end_y {
                        let y = start_y;
                        let start = start_x.min(end_x);
                        let end = start_x.max(end_x);
                        for x in start..=end {
                            println!("  ({x},{y})");
                            rocks.insert(Object::Rock((x, y)));
                        }
                    } else if start_x == end_x {
                        let x = start_x;
                        let start = start_y.min(end_y);
                        let end = start_y.max(end_y);
                        for y in start..=end {
                            println!("  ({x},{y})");
                            rocks.insert(Object::Rock((x, y)));
                        }
                    }
                });
        });
    
    println!("{rocks:?}");
    display(&rocks);


    0
}

// #[aoc(day14, part2)]
// pub fn part2(input: &str) -> u32 {
//     0
// }

type Point = (usize, usize);

#[derive(Debug, Eq, Hash, PartialEq)]
enum Object {
    Rock(Point),
    Sand(Point),
}

impl Object {
    fn x(&self) -> usize {
        match self {
            Object::Rock(r) => r.0,
            Object::Sand(s) => s.0,
        }
    }

    fn y(&self) -> usize {
        match self {
            Object::Rock(r) => r.1,
            Object::Sand(s) => s.1,
        }
    }
}

fn display(objects: &HashSet<Object>) {
    // Searching this way is expensive, but its just for printing to console, so 🤷🏼‍♂️
    let min_x = objects.iter().min_by_key(|o| o.x()).unwrap().x().min(SAND_DROP.0);
    let max_x = objects.iter().max_by_key(|o| o.x()).unwrap().x().max(SAND_DROP.0);
    let min_y = objects.iter().min_by_key(|o| o.y()).unwrap().y().min(SAND_DROP.1);
    let max_y = objects.iter().max_by_key(|o| o.y()).unwrap().y().max(SAND_DROP.1);

    println!("  {:03}{}{:03}", min_x, " ".repeat(max_x - min_x - 1), max_x);

    for y in min_y..=max_y {
        print!("{y:03} ");
        for x in min_x..=max_x {
            if objects.contains(&Object::Rock((x, y))) {
                print!("#");
            } else if objects.contains(&Object::Sand((x, y))) {
                print!("O");
            } else if (x, y) == SAND_DROP {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "498,4 -> 498,6 -> 496,6\n",
        "503,4 -> 502,4 -> 502,9 -> 494,9\n",
    );

    #[test]
    fn part1_sample_input() {
        let answer = part1(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }

    // #[test]
    // fn part2_sample_input() {
    //     let answer = part2(SAMPLE_INPUT);
    //     assert_eq!(answer, 0);
    // }
}
