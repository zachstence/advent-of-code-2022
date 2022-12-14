use std::{fmt::Display, collections::{HashSet, VecDeque}};

use itertools::Itertools;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u32 {
    let mut grid = Grid::parse(input);

    let path = grid.search();

    // println!("{}", grid.show_path_on_grid(&path));

    path.len() as u32
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

    fn get_neighbors(&self, p: &Point) -> Vec<(usize, usize)> {
        let dims = self.get_dimensions();
        let mut neighbors = vec![];

        // If we can go right
        if p.0 < dims.0 - 1 {
            neighbors.push((p.0 + 1, p.1));
        }

        // If we can go down
        if p.1 < dims.1 - 1 {
            neighbors.push((p.0, p.1 + 1));
        }

        // If we can go left
        if p.0 >= 1 {
            neighbors.push((p.0 - 1, p.1));
        }
        
        // If we can go up
        if p.1 >= 1 {
            neighbors.push((p.0, p.1 - 1));
        }

        neighbors
    }

    fn get_elevation_at(&self, p: &Point) -> Option<&char> {
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

    fn elevation_diff(&self, p1: &Point, p2: &Point) -> i32 {
        let _e1 = self.get_elevation_at(p1);
        let _e2 = self.get_elevation_at(p2);

        if _e1.is_none() || _e2.is_none() {
            return i32::MAX;
        }
        let e1 = _e1.unwrap();
        let e2 = _e2.unwrap();
        
        (*e1 as i32) - (*e2 as i32)
    }

    pub fn search(&mut self) -> Path {
        let mut batches: Vec<Vec<Point>> = vec![];

        let mut visited: HashSet<Point> = HashSet::new();
        let mut to_visit: VecDeque<Point> = VecDeque::from([self.start]);

        // To debug further, we probably need to check out the path we're getting
        
        // In order to get the path, each batch we should keep track of what points are visited
        // And then at the end, maybe we can find the completed path between those batches?

        while !to_visit.is_empty() {
            // Visit points in batches
            let mut visiting = to_visit.clone();
            
            let batch: Vec<Point> = Vec::from(to_visit.clone());
            batches.push(batch);
        
            to_visit.drain(..);

            while !visiting.is_empty() {
                let curr = visiting.pop_front().unwrap();
                visited.insert(curr);

                if curr == self.end {
                    to_visit.drain(..);
                    break;
                }

                let neighbors_to_visit = self.get_neighbors(&curr)
                    .into_iter()
                    .filter(|n| {
                        let is_visited = visited.contains(n);
                        let is_going_to_visit = to_visit.contains(n) || visiting.contains(n);
                        let accessible = self.can_you_go_from_p1_to_p2(&curr, n);
                        !is_visited && !is_going_to_visit && accessible
                    })
                    .collect::<Vec<Point>>();

                neighbors_to_visit.iter().for_each(|n| to_visit.push_back(*n));
                            }
        }

        self.build_path_from_batches(&batches)
    }

    fn build_path_from_batches(&self, batches: &[Vec<Point>]) -> Path {
        let mut path: Path = vec![];

        let mut curr: Point = self.end;
        for (prev_batch, _) in batches.iter().rev().tuple_windows() {

            let next = prev_batch
                .iter()
                .find(|p| self.can_you_go_from_p1_to_p2(p, &curr))
                .unwrap_or_else(|| panic!("{curr:?} should have a neighbor in {prev_batch:?}"));
            
            path.push(*next);
            curr = *next;
        }

        path.reverse();
        path
    }

    fn can_you_go_from_p1_to_p2(&self, p1: &Point, p2: &Point) -> bool {
        let are_horizontal_neighbors = (-1..=1).contains(&(p1.0 as i32 - p2.0 as i32)) && p1.1 == p2.1;
        let are_vertical_neighbors = (-1..=1).contains(&(p1.1 as i32 - p2.1 as i32)) && p1.0 == p2.0;
        let are_neighbors = are_horizontal_neighbors || are_vertical_neighbors;

        if !are_neighbors {
            return false;
        }
        
        let diff = self.elevation_diff(p2, p1);
        diff <= 1
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
                    _ => panic!("Unexpected delta {delta:?} between {curr:?} and {next:?}"),
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

    #[test]
    fn part1_sample_input() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";

        let answer = part1(input);
        assert_eq!(answer, 31);
    }

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
        assert_eq!(answer, 43);
    }

    #[test]
    fn test_elevation_diff_up() {
        let input = "SE";

        let grid = Grid::parse(input);
        let diff = grid.elevation_diff(&(0, 0), &(1, 0));

        assert_eq!(diff, -25);
    }

    #[test]
    fn test_elevation_diff_down() {
        let input = "ES";

        let grid = Grid::parse(input);
        let diff = grid.elevation_diff(&(0, 0), &(1, 0));

        assert_eq!(diff, 25);
    }

    #[test]
    fn test_elevation_diff_same() {
        let input = "SaE";

        let grid = Grid::parse(input);
        let diff = grid.elevation_diff(&(0, 0), &(1, 0));

        assert_eq!(diff, 0);
    }
}
