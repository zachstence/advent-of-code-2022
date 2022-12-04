#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(get_bounds)
        .filter(has_contain)
        .count()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(get_bounds)
        .filter(has_overlap)
        .count()
}


fn get_bounds(line: &str) -> [i32; 4] {
    line
        .split(&[',', '-'])
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()[..4]
        .try_into()
        .unwrap()
}

fn has_contain(bounds: &[i32; 4]) -> bool {
    let [elf0_start, elf0_end, elf1_start, elf1_end] = bounds;
    let elf0_sectors = elf0_end - elf0_start;
    let elf1_sectors = elf1_end - elf1_start;

    if elf0_sectors > elf1_sectors { // Elf 0 could contain elf1
        elf0_start <= elf1_start && elf0_end >= elf1_end
    } else { // Elf 1 could contain Elf 0
        elf1_start <= elf0_start && elf1_end >= elf0_end
    }
}

fn has_overlap(bounds: &[i32; 4]) -> bool {
    let [elf0_start, elf0_end, elf1_start, elf1_end] = bounds;
    
    let elf0_start_inside_elf1 = elf0_start >= elf1_start && elf0_start <= elf1_end;
    let elf0_end_inside_elf1 = elf0_end >= elf1_start && elf0_end <= elf1_end;
    let elf1_start_inside_elf0 = elf1_start >= elf0_start && elf1_start <= elf0_end;
    let elf1_end_inside_elf0 = elf1_end >= elf0_start && elf1_end <= elf0_end;

    elf0_start_inside_elf1 || elf0_end_inside_elf1 || elf1_start_inside_elf0 || elf1_end_inside_elf0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_overlap() {
        assert!(!has_overlap(&[1, 2, 3, 4]));
        assert!(!has_overlap(&[3, 4, 1, 2]));
        assert!(!has_overlap(&[1, 1, 2, 2]));
        assert!(!has_overlap(&[2, 2, 1, 1]));
        assert!(has_overlap(&[1, 1, 1, 1]));
        assert!(has_overlap(&[1, 10, 5, 5]));
        assert!(has_overlap(&[5, 5, 1, 10]));
        assert!(has_overlap(&[1, 2, 2, 3]));
        assert!(has_overlap(&[2, 3, 1, 2]));
        assert!(has_overlap(&[1, 3, 2, 4]));
        assert!(has_overlap(&[2, 4, 1, 3]));
        assert!(has_overlap(&[1, 4, 2, 3]));
        assert!(has_overlap(&[2, 3, 1, 4]));
    }
}