use std::fmt;

use itertools::Itertools;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut cycle: u32 = 0;
    let mut reg: i32 = 1;

    input
        .lines()
        .map(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let instruction = tokens[0];


            let instruction_cycles = match instruction {
                "noop" => 1,
                "addx" => 2,
                _ => panic!("Unknown instruction {}", instruction),
            };

            (0..instruction_cycles).map(|c| {
                cycle += 1;

                let ss = get_signal_strength(cycle, reg);

                match (instruction, c) {
                    ("noop", 0) => (),
                    ("noop", _) => panic!("noop instruction executing longer than 1 cycle"),

                    ("addx", 0) => (),
                    ("addx", 1) => {
                        let val = tokens[1].parse::<i32>().expect("addx arg should be an i32");
                        reg += val;
                    }
                    ("addx", _) => panic!("addx instruction executing longer than 2 cycles"),

                    (_, _) => panic!("Unknown instruction {}", instruction),
                }

                if is_40c_plus_20(cycle) {
                    ss
                } else {
                    0
                }
            })
            .sum::<i32>()
        })
        .sum()
}


#[aoc(day10, part2)]
fn part2(input: &str) -> String {

    let mut screen = Screen::new();

    let mut cycle: usize = 0;
    let mut reg: i32 = 1;

    input
        .lines()
        .for_each(|line| {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let instruction = tokens[0];


            let instruction_cycles = match instruction {
                "noop" => 1,
                "addx" => 2,
                _ => panic!("Unknown instruction {}", instruction),
            };

            for c in 0..instruction_cycles {
                cycle += 1;



                // Sprite is 3 pixels wide, so check one below and above reg as well
                let pixel = cycle - 1;
                let line_pixel = pixel % 40;
                let should_draw = (reg - 1..reg + 2).contains(&(line_pixel as i32));
                if should_draw {
                    screen.draw_pixel(pixel);
                }

                match (instruction, c) {
                    ("noop", 0) => (),
                    ("noop", _) => panic!("noop instruction executing longer than 1 cycle"),

                    ("addx", 0) => (),
                    ("addx", 1) => {
                        let val = tokens[1].parse::<i32>().expect("addx arg should be an i32");
                        reg += val;
                    }
                    ("addx", _) => panic!("addx instruction executing longer than 2 cycles"),

                    (_, _) => panic!("Unknown instruction {}", instruction),
                }
            }
        });

    screen.to_string()
}


///////////////


struct Screen {
    pixels: [bool; 240]
}

impl Screen {
    fn new() -> Screen {
        Screen {
            pixels: [false; 240],
        }
    }

    fn draw_pixel(&mut self, i: usize) {
        if (0..240).contains(&i) {
            self.pixels[i] = true
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.pixels
            .chunks(40)
            .map(|line| line.iter().map(|pixel| if *pixel { '█' } else { ' ' }).join("") + "\n").join("");
        write!(f, "\n{}", s)
    }
}

fn is_40c_plus_20(cycle: u32) -> bool {
    cycle >= 20 && (cycle - 20) % 40 == 0
}

fn get_signal_strength(cycle: u32, reg: i32) -> i32 {
    (cycle as i32) * reg
}

#[cfg(test)]
mod day10_tests{
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

        let answer = part1(input);
        assert_eq!(answer, 13140);
    }

    #[test]
    fn part2_sample_input() {
        let input = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

        let answer = part2(input);
        println!("{}", answer);
    }
}