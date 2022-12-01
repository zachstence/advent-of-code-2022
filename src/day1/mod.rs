use std::fs;

pub fn part1(input_file: &str) {
    let input = fs::read_to_string(input_file)
        .expect("Should be able to read input file");
    println!("{input}")
}
