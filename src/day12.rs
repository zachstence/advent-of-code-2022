use std::fmt::Display;

use itertools::Itertools;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    println!("{grid}");

    let path = grid.search();
    println!("\n=======\n{path:?}");
    println!("{}", grid.show_path_on_grid(&path));

    path.len() - 1
}

type Point = (usize, usize);

type Path = Vec<Point>;

struct Grid {
    grid: Vec<Vec<char>>,
    start: Point,
    end: Point,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut _start: Option<Point> = None;
        let mut _end: Option<Point> = None;

        let grid = input
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.chars().enumerate().map(|(c, mut ch)| {
                    if ch == 'S' {
                        _start = Some((c, r));
                        ch = 'a';
                    } else if ch == 'E' {
                        _end = Some((c, r));
                        ch = 'z';
                    }
                    ch
                }).collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();
        
        let start = _start.expect("Should find start position denoted by 'S'");
        let end = _end.expect("Should find end position denoted by 'E'");

        Self { grid, start, end }
    }

    fn get_num_rows(&self) -> usize {
        self.grid.len()
    }

    fn get_num_cols(&self) -> usize {
        if let Some(first_row) = self.grid.first() {
            first_row.len()
        } else {
            0
        }
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.get_num_cols(), self.get_num_rows())
    }

    fn get_elevation_at(&self, p: Point) -> Option<&char> {
        match self.grid.get(p.1) {
            Some(row) => {
                match row.get(p.0) {
                    Some(e) => Some(e),
                    None => None,
                }
            },
            None => None,
        }
    }

    fn elevation_diff(&self, p1: Point, p2: Point) -> i32 {
        let _e1 = self.get_elevation_at(p1);
        let _e2 = self.get_elevation_at(p2);

        if _e1.is_none() || _e2.is_none() {
            return i32::MAX;
        }
        let e1 = _e1.unwrap();
        let e2 = _e2.unwrap();
        
        (*e1 as i32) - (*e2 as i32)
    }

    pub fn search(&mut self) -> Vec<Point> {
        self._search(self.start, vec![]).expect("A path from Start to End should exist")
    }

    fn _search(&mut self, p: Point, mut path: Path) -> Option<Path> {
        println!();
        println!("_search {p:?} -> {:?}, current path with length {} : {path:?}", self.end, path.len());
        path.push(p);

        if p.0 == self.end.0 && p.1 == self.end.1 {
            println!("Got to the end! {path:?}");
            return Some(path.to_vec());
        }

        // Determine what neighbors we should visit
        let dims = self.get_dimensions();
        let mut neighbors = vec![];

        // If we can go right
        if p.0 < dims.0 - 1 {
            let right = (p.0 + 1, p.1);
            let diff = self.elevation_diff(right, p);
            // Make sure that point isn't in our path already and that it is reachable
            if !path.contains(&right) && diff <= 1 {
                neighbors.push(right);
            }
        }

        // If we can go down
        if p.1 < dims.1 - 1 {
            let down = (p.0, p.1 + 1);
            let diff = self.elevation_diff(down, p);
            // Make sure that point isn't in our path already and that it is reachable
            if !path.contains(&down) && diff <= 1 {
                neighbors.push(down);
            }
        }

        // If we can go left
        if p.0 >= 1 {
            let left = (p.0 - 1, p.1);
            let diff = self.elevation_diff(left, p);
            // Make sure that point isn't in our path already and that it is reachable
            if !path.contains(&left) && diff <= 1 {
                neighbors.push(left);
            }
        }
        
        // If we can go up
        if p.1 >= 1 {
            let up = (p.0, p.1 - 1);
            let diff = self.elevation_diff(up, p);
            // Make sure that point isn't in our path already and that it is reachable
            if !path.contains(&up) && diff <= 1 {
                neighbors.push(up);
            }
        }

        println!("Visiting {} neighbors {neighbors:?}", neighbors.len());

        if neighbors.len() == 0 {
            println!("Reached dead end at {p:?} with path {path:?}");
        }

        let paths = neighbors
            .iter()
            .filter_map(|n| self._search(*n, path.clone()))
            .collect::<Vec<Path>>();
                
        let shortest_path = paths.iter().fold(None, |shortest, path| {
                if shortest.is_none() {
                    return Some(path);
                }

                let shortest_len = shortest.unwrap().len();
                if path.len() < shortest_len {
                    return Some(path);
                }

                shortest
            });
        
        shortest_path.cloned()
    }

    fn show_path_on_grid(&self, path: &Path) -> String {
        let mut grid = self.grid.clone()
            .iter_mut()
            .map(|row| row.iter_mut().map(|_| '.').collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        path
            .iter()
            .tuple_windows()
            .for_each(|(curr, next)| {
                let delta = (next.0 as i32 - curr.0 as i32, next.1 as i32 - curr.1 as i32);
                let ch = match delta {
                    (1, 0) => '→',
                    (-1, 0) => '←',
                    (0, 1) => '↓',
                    (0, -1) => '↑',
                    _ => {
                        // println!("Unexpected delta {delta:?} between {curr:?} and {next:?}");
                        '?'
                    },
                };
                grid[curr.1][curr.0] = ch;
            });
        
        grid[self.start.1][self.start.0] = 'S';
        grid[self.end.1][self.end.0] = 'E';
        
        grid_to_string(&grid)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Start={:?}\tEnd={:?}\n{}",
            self.start,
            self.end,
            grid_to_string(&self.grid),
        )
    }
}

fn grid_to_string(grid: &[Vec<char>]) -> String {
    grid.iter().map(|row| row.iter().collect::<String>()).join("\n")
}

#[cfg(test)]
mod day12_tests {
    use super::*;

    // #[test]
    // fn part1_sample_input() {
    //     let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";

    //     let answer = part1(input);
    //     assert_eq!(answer, 31);
    // }

    #[test]
    fn custom() {
        /*
        abcccdeeefff
        abcaaaeeeegg
        abccaefyyzeg
        SabcaazEwyxf
        abcdopqreewg
        abdnnmmstuvh
        abdnnmmmlkji
        */
        let input = "abcccdeeefff\nabcaaaeeeegg\nabccaefyyzeg\nSabcaazEwyxf\nabcdopqreewg\nabdnnmmstuvh\nabdnnmmmlkji\n";

        let answer = part1(input);
        assert_eq!(answer, 40);
    }
}
