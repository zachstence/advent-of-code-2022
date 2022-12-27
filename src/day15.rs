use lazy_static::lazy_static;
use regex::Regex;

const BEACON: char = 'B';
const SENSOR: char = 'S';
const NOT_BEACON: char = '#';

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$").unwrap();
}

type Point = (i32, i32);

#[derive(Debug)]
pub struct ParsedLine {
    sensor: Point,
    closest_beacon: Point,
}

type Input = Vec<ParsedLine>;

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| LINE_REGEX.captures_iter(line).collect::<Vec<_>>())
        .map(|captures| {
            println!("{captures:?}");
            let sensor_x = captures[0][1].parse::<i32>().unwrap();
            let sensor_y = captures[0][2].parse::<i32>().unwrap();
            let beacon_x = captures[0][3].parse::<i32>().unwrap();
            let beacon_y = captures[0][4].parse::<i32>().unwrap();
            ParsedLine { sensor: (sensor_x, sensor_y), closest_beacon: (beacon_x, beacon_y) }
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> u32 {
    // Read in sensors and beacons, and mark where beacons can't be
    println!("{input:?}");

    0
}

#[aoc(day15, part2)]
pub fn part2(input: &Input) -> u32 {
    0
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

    #[test]
    fn part2_sample_input() {
        let input = generator(SAMPLE_INPUT);
        let answer = part2(&input);
        assert_eq!(answer, 0);
    }
}
