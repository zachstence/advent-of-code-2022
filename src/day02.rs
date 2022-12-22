#[allow(clippy::get_first)]

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    input.lines().map(|l| part1_get_score(l.split_once(' ').unwrap())).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    input.lines().map(|l| part2_get_score(l.split_once(' ').unwrap())).sum()
}

const LOSS_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;
const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;

fn part1_get_score(moves: (&str, &str)) -> i32 {
    match moves {
        // Losses
        ("B", "X") => LOSS_SCORE + ROCK_SCORE,
        ("C", "Y") => LOSS_SCORE + PAPER_SCORE,
        ("A", "Z") => LOSS_SCORE + SCISSORS_SCORE,

        // Draws
        ("A", "X") => DRAW_SCORE + ROCK_SCORE,
        ("B", "Y") => DRAW_SCORE + PAPER_SCORE,
        ("C", "Z") => DRAW_SCORE + SCISSORS_SCORE,

        // Wins
        ("C", "X") => WIN_SCORE + ROCK_SCORE,
        ("A", "Y") => WIN_SCORE + PAPER_SCORE,
        ("B", "Z") => WIN_SCORE + SCISSORS_SCORE,

        _ => panic!("Unknown moves {} {}", moves.0, moves.1)
    }
}

fn part2_get_score(moves: (&str, &str)) -> i32 {
    match moves {
        // Losses
        ("A", "X") => LOSS_SCORE + SCISSORS_SCORE,
        ("B", "X") => LOSS_SCORE + ROCK_SCORE,
        ("C", "X") => LOSS_SCORE + PAPER_SCORE,

        // Draws
        ("A", "Y") => DRAW_SCORE + ROCK_SCORE,
        ("B", "Y") => DRAW_SCORE + PAPER_SCORE,
        ("C", "Y") => DRAW_SCORE + SCISSORS_SCORE,
        
        // Wins
        ("A", "Z") => WIN_SCORE + PAPER_SCORE,
        ("B", "Z") => WIN_SCORE + SCISSORS_SCORE,
        ("C", "Z") => WIN_SCORE + ROCK_SCORE,

        _ => panic!("Unknown moves {} {}", moves.0, moves.1)
    }
}

