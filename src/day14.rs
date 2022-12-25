use std::{collections::HashSet};

use itertools::Itertools;

const SAND_DROP: Point = (500, 0);
const MAX_ITERS: usize = 1;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut rocks: HashSet<Object> = HashSet::new();

    let mut min_x = SAND_DROP.0;
    let mut min_y = SAND_DROP.1;
    let mut max_x = SAND_DROP.0;
    let mut max_y = SAND_DROP.1;

    input
        .lines()
        .for_each(|line| {
            line
                .split(" -> ")
                .map(|token| token.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .tuple_windows()
                .for_each(|((start_x, start_y), (end_x, end_y))| {
                    min_x = min_x.min(start_x).min(end_x);
                    max_x = max_x.max(start_x).max(end_x);
                    min_y = min_y.min(start_y).min(end_y);
                    max_y = max_y.max(start_y).max(end_y);

                    if start_y == end_y {
                        let y = start_y;
                        let start = start_x.min(end_x);
                        let end = start_x.max(end_x);
                        for x in start..=end {
                            rocks.insert(Object::Rock((x, y)));
                        }
                    } else if start_x == end_x {
                        let x = start_x;
                        let start = start_y.min(end_y);
                        let end = start_y.max(end_y);
                        for y in start..=end {
                            rocks.insert(Object::Rock((x, y)));
                        }
                    }
                });
        });
    
    display(&rocks, &(min_x, min_y), &(max_x, max_y));

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

fn display(objects: &HashSet<Object>, min: &Point, max: &Point) {
    println!("  {:03}{}{:03}", min.0, " ".repeat(max.0 - min.0 - 1), max.0);

    for y in min.1..=max.1 {
        print!("{y:03} ");
        for x in min.0..=max.0 {
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
