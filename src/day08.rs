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
            if is_visible(&trees, r, c) {
                visible += 1;
            }
        }
    }

    visible
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
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

    let mut max_scenic_score = 0_usize;
    for r in 1..num_rows - 1 {
        for c in 1..num_cols - 1 {
            max_scenic_score = max_scenic_score.max(scenic_score(&trees, r, c));
        }
    }

    max_scenic_score
}

#[allow(clippy::ptr_arg)]
fn is_visible(trees: &Vec<Vec<u32>>, r: usize, c: usize) -> bool {
    let tree = trees[r][c];

    let top_max = trees[0..r].iter().map(|row| row[c]).max().unwrap();
    if top_max < tree {
        return true;
    }

    let bottom_max = trees[r + 1..].iter().map(|row| row[c]).max().unwrap();
    if bottom_max < tree {
        return true;
    }

    let left_max = *trees[r].iter().take(c).max().unwrap();
    if left_max < tree {
        return true;
    }

    let right = *trees[r].iter().skip(c + 1).max().unwrap();
    if right < tree {
        return true;
    }

    false
}

#[allow(clippy::ptr_arg)]
fn scenic_score(trees: &Vec<Vec<u32>>, r: usize, c: usize) -> usize {
    let tree = trees[r][c];

    let mut visible_top = 0;
    for height in trees[0..r].iter().map(|row| row[c]).rev() {
        visible_top += 1;
        if height >= tree { break; }
    }

    let mut visible_bottom = 0;
    for height in trees[r + 1..].iter().map(|row| row[c]) {
        visible_bottom += 1;
        if height >= tree { break; }
    }

    let mut visible_left = 0;
    for height in trees[r].iter().take(c).rev() {
        visible_left += 1;
        if *height >= tree { break; }
    }

    let mut visible_right = 0;
    for height in trees[r].iter().skip(c + 1) {
        visible_right += 1;
        if *height >= tree { break; }
    }

    visible_top * visible_bottom * visible_left * visible_right
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

    #[test]
    fn part2_sample_input() {
        let answer = part2(SAMPLE_INPUT);
        assert_eq!(answer, 8);
    }
}
