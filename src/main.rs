use clap::Parser;

mod day1;
mod day2;

#[derive(Parser)]
struct Args {
    day: i32,
    part: i32,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let part = args.part;

    let mut answer = String::from("");

    if day == 1 {
        if part == 1 {
            answer = day1::part1("./src/day1/input.txt");
        } else if part == 2 {
            answer = day1::part2("./src/day1/input.txt");
        }
    } else if day == 2 {
        if part == 1 {
            answer = day2::part1("./src/day2/input.txt");
        } else if part == 2 {
            answer = day2::part2("./src/day2/test.txt");
        }
    }

    println!("Day {day}, part {part}");
    println!("---");
    println!("Answer: {answer}");
}
