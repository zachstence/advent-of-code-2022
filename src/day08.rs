#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let trees = input
        .lines()
        .map(|line| line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>();
    
    let num_rows = trees.len();
    let num_cols = trees[0].len();

    let mut visible = (num_rows - 1) * 2 + (num_cols - 1) * 2;
    
    for r in 1..num_rows - 1 {
        for c in 1..num_cols - 1 {
            let tree = trees[r][c];

            let top_max = trees[0..r].iter().map(|row| row[c]).max().unwrap();
            if top_max < tree {
                visible += 1;
                continue;
            }

            let bottom_max = trees[r + 1..num_rows].iter().map(|row| row[c]).max().unwrap();
            if bottom_max < tree {
                visible += 1;
                continue;
            }

            let left_max = *trees[r].iter().take(c).max().unwrap();
            if left_max < tree {
                visible += 1;
                continue;
            }

            let right = *trees[r].iter().skip(c + 1).max().unwrap();
            if right < tree {
                visible += 1;
                continue;
            }
        }
    }

    visible
}

#[cfg(test)]
mod day8_tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "30373\n",
        "25512\n",
        "65332\n",
        "33549\n",
        "35390",
    );

    #[test]
    fn part1_sample_input() {
        let answer = part1(SAMPLE_INPUT);
        assert_eq!(answer, 21);
    }
}
