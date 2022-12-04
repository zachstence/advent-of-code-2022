#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(get_bounds)
        .map(|bounds| {
            let [elf0_start, elf0_end, elf1_start, elf1_end] = bounds;
            let elf0_sectors = elf0_end - elf0_start;
            let elf1_sectors = elf1_end - elf1_start;

            if elf0_sectors > elf1_sectors { // Elf 0 could contain elf1
                elf0_start <= elf1_start && elf0_end >= elf1_end
            } else { // Elf 1 could contain Elf 0
                elf1_start <= elf0_start && elf1_end >= elf0_end
            }
        })
        .filter(|contains| *contains)
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
