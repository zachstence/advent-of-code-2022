mod filesystem;
use filesystem::Filesystem;

mod lines;
use lines::parse;

use self::filesystem::Directory;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let lines = parse(input);
    let mut fs = Filesystem::new();
    fs.exec_lines(lines);

    for dir in fs.get_directories() {
        let sizes_sum = dir.children().iter().map(|i| fs.get_node(*i).unwrap()).map(|node| node.size()).sum::<u64>();
        assert_eq!(dir.total_size(), sizes_sum);
    }
    
    let sum = fs.get_directories()
        .iter()
        .filter_map(|dir| {
            let size = dir.total_size();
            if size <= 100_000 {
                Some(size)
            } else {
                None
            }
        })
        .sum::<u64>();

    sum
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let disk_space = 70_000_000;
    let space_needed = 30_000_000;

    let lines = parse(input);
    let mut fs = Filesystem::new();
    fs.exec_lines(lines);

    let total_used = fs.root_dir().total_size();
    let unused = disk_space - total_used;
    let to_free = space_needed - unused;

    let dirs = fs.get_directories();
    let mut candidates = dirs
        .iter()
        .filter(|dir| dir.total_size() >= to_free)
        .collect::<Vec<&&Directory>>();
        
    candidates.sort_by_key(|a| a.total_size());

    candidates.first().unwrap().total_size()
}


#[cfg(test)]
pub mod day7_tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let answer = part1(input);
        assert_eq!(answer, 95437);
    }

    #[test]
    fn part1_deeply_nested() {
        let input = [
            "$ cd /",
            "$ cd /",
            "$ cd /",
            "$ cd /",
            "$ cd /",
            "$ ls",
            "dir a",
            "$ cd a",
            "$ ls",
            "dir b",
            "$ cd b",
            "$ ls",
            "dir c",
            "$ cd c",
            "$ ls",
            "dir d",
            "$ cd d",
            "$ ls",
            "dir e",
            "$ cd e",
            "$ cd ..",
            "$ cd e",
            "$ ls",
            "dir f",
            "$ cd f",
            "$ ls",
            "dir g",
            "$ cd g",
            "$ ls",
            "dir h",
            "$ cd h",
            "$ ls",
            "1 file.txt",
        ].join("\n");

        let answer = part1(&input);
        assert_eq!(answer, 9);
    }

    #[test]
    fn part1_should_include_100000() {
        let input = [
            "$ cd /",
            "$ ls",
            "100000 file.txt",
        ].join("\n");

        let answer = part1(&input);

        assert_eq!(100000, answer);
    }

    #[test]
    fn part1_dirs_with_0_size() {
        let input = [
            "$ cd /",
            "$ ls",
            "dir a1",
            "dir a2",
            "$ cd a2",
            "$ ls",
            "1234 file",
            "dir b3",
            "$ cd b3",
            "$ ls",
        ].join("\n");

        let answer = part1(&input);

        assert_eq!(answer, 1234 * 2);
    }
}
