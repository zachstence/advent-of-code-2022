use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CD_REGEX: Regex = Regex::new(r"^\$ cd (.*)$").unwrap();
    static ref LS_REGEX: Regex = Regex::new(r"^\$ ls$").unwrap();
    static ref DIR_REGEX: Regex = Regex::new(r"^dir (.*)$").unwrap();
    static ref FILE_REGEX: Regex = Regex::new(r"^(\d+) (.*)$").unwrap();
}


#[derive(Debug)]
pub struct ChangeDirectoryLine {
    dir_name: String,
}

impl TryFrom<&str> for ChangeDirectoryLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let dir_name = CD_REGEX
            .captures(value).ok_or("Parsing ChangeDirectoryLine gave no captures")?
            .get(1).ok_or("Parsed ChangeDirectoryLine missing name")?
            .as_str()
            .to_string();

        Ok(ChangeDirectoryLine { dir_name })
    }
}

#[derive(Debug)]
pub struct ListLine {}

impl TryFrom<&str> for ListLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        LS_REGEX
            .captures(value).ok_or("Parsing ListLine gave no captures")?;
        
        Ok(ListLine {})
    }
}

#[derive(Debug)]
pub struct DirectoryLine {
    name: String,
}

impl TryFrom<&str> for DirectoryLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let name = DIR_REGEX
            .captures(value).ok_or("Parsing DirectoryLine gave no captures")?
            .get(1).ok_or("Parsed DirectoryLine missing name")?
            .as_str()
            .to_string();
        
        Ok(DirectoryLine { name })
    }
}


#[derive(Debug)]
pub struct FileLine {
    name: String,
    size: u32,
}

impl TryFrom<&str> for FileLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let size = FILE_REGEX
            .captures(value).ok_or("Parsing FileLine gave no captures")?
            .get(1).ok_or("Parsed FileLine missing size")?
            .as_str()
            .parse::<u32>().or(Err("Unable to parse file size from parsed FileLine"))?;
        let name = FILE_REGEX
            .captures(value).ok_or("Parsing FileLine gave no captures")?
            .get(2).ok_or("Parsed FileLine missing name")?
            .as_str()
            .to_string();

        Ok(FileLine { name, size })
    }
}

#[derive(Debug)]
pub enum ParsedLine {
    ChangeDirectory(ChangeDirectoryLine),
    List(ListLine),
    Directory(DirectoryLine),
    File(FileLine),
}

impl TryFrom<&str> for ParsedLine {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(parsed) = ChangeDirectoryLine::try_from(value) {
            Ok(ParsedLine::ChangeDirectory(parsed))
        } else if let Ok(parsed) = ListLine::try_from(value) {
            Ok(ParsedLine::List(parsed))
        } else if let Ok(parsed) = DirectoryLine::try_from(value) {
            Ok(ParsedLine::Directory(parsed))
        } else if let Ok(parsed) = FileLine::try_from(value) {
            Ok(ParsedLine::File(parsed))
        } else {
            Err("Unable to parse line")
        }
    }
}


pub fn parse(input: &str) -> Vec<ParsedLine> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> ParsedLine {
    match ParsedLine::try_from(line) {
        Ok(parsed) => parsed,
        Err(e) => panic!("Unable to parse line '{line}' : {e}"),
    }
}

#[cfg(test)]
pub mod day7_lines_tests {
    use super::*;

    #[test]
    fn parse_cd_line() {
        let line = "$ cd /";
        let expected = ChangeDirectoryLine { dir_name: "/".to_string() };

        let actual = ChangeDirectoryLine::try_from(line);

        match actual {
            Ok(parsed) => assert_eq!(expected.dir_name, parsed.dir_name),
            Err(e) => panic!("{e}"),
        }
    }

    #[test]
    fn parse_file_line() {
        let line = "227398 rwhw";
        let expected = FileLine { name: "rwhw".to_string(), size: 227398 };

        let actual = FileLine::try_from(line);

        match actual {
            Ok(parsed) => {
                assert_eq!(expected.name, parsed.name);
                assert_eq!(expected.size, parsed.size);
            }
            Err(e) => panic!("{e}"),
        }
    }
}