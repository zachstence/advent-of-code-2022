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
}