#[aoc(dayXX, part1)]
pub fn part1(input: &str) -> u32 {
    0
}

#[aoc(dayXX, part2)]
pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod dayXX_tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "line1\n",
        "line2\n",
        "line3\n",
    );

    #[test]
    fn part1_sample_input() {
        let answer = part1(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }

    #[test]
    fn part2_sample_input() {
        let answer = part2(SAMPLE_INPUT);
        assert_eq!(answer, 0);
    }
}
