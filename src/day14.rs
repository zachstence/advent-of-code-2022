use std::{collections::HashSet};

use itertools::Itertools;

const SAND_DROP: Point = (500, 0);
const DX: &[i32] = &[0, -1, 1];
const MAX_SAND: usize = 100_000;

type Input = (HashSet<Point>, Point, Point);

/// Returns Rock locations, min point, and max point 
#[aoc_generator(day14)]
pub fn generator(input: &str) -> Input {
    let mut rocks: HashSet<Point> = HashSet::new();

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
                .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
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
                            rocks.insert((x, y));
                        }
                    } else if start_x == end_x {
                        let x = start_x;
                        let start = start_y.min(end_y);
                        let end = start_y.max(end_y);
                        for y in start..=end {
                            rocks.insert((x, y));
                        }
                    }
                });
        });
    
    (rocks, (min_x, min_y), (max_x, max_y))
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> usize {
    let (rocks, _, (_, max_y)) = input;

    // Rocks and Sand could be kept in the same HashSet, but having them separate allows us to display them
    let mut sand: HashSet<Point> = HashSet::new();

    // Prevent infinite loop from a bug
    for _ in 0..MAX_SAND {
        let mut x = SAND_DROP.0;
        let mut y = SAND_DROP.1;
        while y != *max_y {
            if let Some(dx) = DX.iter().find(|dx| !is_occupied(rocks, &sand, &(x + *dx, y + 1))) {
                // Falls to (x,y)
                x += *dx;
                y += 1;
                continue;
            } else {
                // Can't fall, is at rest
                sand.insert((x, y));
                break;
            }
        }

        // Sand has fallen to the abyss, we're done simulating
        if y == *max_y { break; }
    }

    sand.len()
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> usize {
    let (rocks, _, (_, max_y)) = input;

    // Rocks and Sand could be kept in the same HashSet, but having them separate allows us to display them
    let mut sand: HashSet<Point> = HashSet::new();

    // Prevent infinite loop from a bug
    for _ in 0..MAX_SAND {
        let mut x = SAND_DROP.0;
        let mut y = SAND_DROP.1;
        while y != *max_y + 1 {
            if let Some(dx) = DX.iter().find(|dx| !is_occupied(rocks, &sand, &(x + *dx, y + 1))) {
                // Falls to (x,y)
                x += *dx;
                y += 1;
                continue;
            } else {
                // Can't fall, is at rest
                sand.insert((x, y));
                break;
            }
        }

        // Sand has fallen to the bottom, done falling
        if y == *max_y + 1 {
            sand.insert((x, y));
            continue;
        }

        // Sand has piled up to the top, done simulating
        if y == SAND_DROP.1 { break; }
    }

    sand.len()
}

type Point = (i32, i32);

#[allow(dead_code)]
fn display(rocks: &HashSet<Point>, sand: &HashSet<Point>, min: &Point, max: &Point, bottom: bool) {
    // println!("  {:03}{}{:03}", min.0 - 5, " ".repeat((max.0 - min.0 + 9) as usize), max.0 + 5);

    for y in min.1..=max.1 + 2 {
        print!("{y:03} ");
        for x in min.0 - 5..=max.0 + 5 {
            if rocks.contains(&(x, y)) {
                print!("#");
            } else if sand.contains(&(x, y)) {
                print!("o");
            } else if (x, y) == SAND_DROP {
                print!("+");
            } else if bottom && y == max.1 + 2 {
                print!("#");
            } else {
                print!(".");
            }
        }
        // println!();
    }
}

fn is_occupied(rocks: &HashSet<Point>, sand: &HashSet<Point>, point: &Point) -> bool {
    rocks.contains(point) || sand.contains(point)
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
        let input = generator(SAMPLE_INPUT);
        let answer = part1(&input);
        assert_eq!(answer, 24);
    }

    #[test]
    fn part2_sample_input() {
        let input = generator(SAMPLE_INPUT);
        let answer = part2(&input);
        assert_eq!(answer, 93);
    }
}
