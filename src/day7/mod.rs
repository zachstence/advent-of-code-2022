mod filesystem;
use filesystem::Filesystem;

mod lines;
use lines::parse;

use self::lines::ParsedLine;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {

    let lines = parse(input);

    let mut fs = Filesystem::new();

    lines
        .into_iter()
        .enumerate()
        .for_each(|(i, l)| {
            println!("\n=== Executing line {} ===", i + 1);
            println!("Filesystem:\n{fs}");
            println!("curr_dir: {}", fs.curr_dir());
            println!("Line: {l:?}");

            match l {
                ParsedLine::ChangeDirectory(cd) => fs.cd(&cd.dir_name),
                ParsedLine::Directory(dir) => { fs.add_directory(dir.name); },
                ParsedLine::File(file) => { fs.add_file(file.name, file.size); },
                ParsedLine::List(..) => {},
            }
        });
    
    println!("{fs}");

    0
}


#[cfg(test)]
pub mod day7_tests {
    use super::*;

    #[test]
    fn pass() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let answer = part1(input);
        assert_eq!(answer, 95437);
    }
}
