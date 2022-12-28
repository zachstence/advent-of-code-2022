use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

type Point = (i32, i32);

#[derive(Debug)]
pub struct ParsedLine {
    sensor: Point,
    beacon: Point,
    distance: u32,
}

#[derive(Debug)]
pub struct Input {
    lines: Vec<ParsedLine>,
    min_x: i32,
    max_x: i32,
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Input {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    let lines = input
        .lines()
        .map(|line| LINE_REGEX.captures_iter(line).collect::<Vec<_>>())
        .map(|captures| {
            let sensor_x = captures[0][1].parse::<i32>().unwrap();
            let sensor_y = captures[0][2].parse::<i32>().unwrap();
            let sensor = (sensor_x, sensor_y);

            let beacon_x = captures[0][3].parse::<i32>().unwrap();
            let beacon_y = captures[0][4].parse::<i32>().unwrap();
            let beacon = (beacon_x, beacon_y);

            let distance = manhattan_distance(&sensor, &beacon);

            min_x = min_x.min(sensor_x).min(beacon_x);
            max_x = max_x.max(sensor_x).max(beacon_x);

            ParsedLine { sensor, beacon, distance }
        })
        .collect::<Vec<_>>();
    
    Input { lines, min_x, max_x }
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> usize {
    // Sample input -> 10
    // Actual input -> 2_000_000
    // let line_of_interest = 10;
    let line_of_interest = 2_000_000;

    let mut count = 0_usize;

    // For each point in the line we're looking at
    // TODO my original bounding by min_x and max_x wasn't big enough, how can we bound it correctly?
    for x in input.min_x - 100000000..=input.max_x + 100000000 {
        let point: Point = (x, line_of_interest);

        // Check if this point is covered by another sensor's range
        for line in &input.lines {
            let d = manhattan_distance(&point, &line.sensor);
            
            // If point is already occupied, don't consider it
            if point == line.beacon || point == line.sensor {
                continue;
            }

            // If close to another sensor, beacon can't be here
            if d <= line.distance {
                count += 1;
                break;
            }
        }
    }

    count
}

// #[aoc(day15, part2)]
// pub fn part2(input: &Input) -> u32 {
//     0
// }

fn manhattan_distance(p1: &Point, p2: &Point) -> u32 {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[cfg(test)]
mod day15_tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    );

    #[test]
    fn part1_sample_input() {
        let input = generator(SAMPLE_INPUT);
        let answer = part1(&input);
        assert_eq!(answer, 26);
    }

    // #[test]
    // fn part2_sample_input() {
    //     let input = generator(SAMPLE_INPUT);
    //     let answer = part2(&input);
    //     assert_eq!(answer, 0);
    // }
}
