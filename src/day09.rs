
use std::collections::HashSet;

type Point = (i32, i32);

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    simulate_rope(input, 2)
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    simulate_rope(input, 10)
}


//////////////


fn simulate_rope(input: &str, rope_length: usize) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut knots: Vec<Point> = vec![(0, 0); rope_length];

    // All knots start at (0, 0), so it is visisted
    visited.insert((0, 0));

    input
        .lines()
        .for_each(|line| {
            let (direction, distance) = line.split_once(' ').expect("Line should contain direction and distance");
            let distance = distance.parse::<i32>().expect("Distance should be an unsigned integer");

            #[allow(clippy::single_element_loop)]
            for _ in 0..distance {
                update_positions(direction, &mut knots);

                // Track position of tail knot (last in array)
                visited.insert(*knots.last().unwrap());
            }

        });


    visited.len()
}

fn delta(a: &Point, b: &Point) -> (i32, i32) {
    let dx: i32 = a.0 - b.0;
    let dy: i32 = a.1 - b.1;
    (dx, dy)
}

fn move_point(direction: &str, p: &mut Point) {
    match direction {
        "U" => p.1 += 1,
        "R" => p.0 += 1,
        "D" => p.1 -= 1,
        "L" => p.0 -= 1,
        _ => panic!("Invalid direction {}", direction)
    }
}

fn update_positions(direction: &str, knots: &mut [Point]) {
    let head = knots.get_mut(0).unwrap();

    // First move head
    move_point(direction, head);

    // Move other knots to catch up
    let mut prev = *head;

    for knot in knots[1..].iter_mut() {

        let (dx, dy) = delta(&prev, knot);

        match (dx, dy) {
            // Tail is in the same spot as head
            // No move necessary
            (0, 0) => (),
    
            // Tail is adjacent to head and in the same row/column
            // No move necessary
            (0, -1) | (0, 1) | (-1, 0) | (1, 0) => (),
            
            // Tail is adjacent to head and catty-corner
            // No move necessary
            (-1, -1) | (-1, 1) | (1, -1) | (1, 1) => (),
    
            // Tail is not adjacent, but in the same row column
            // To catch up, we move URDL
            (0, 2) => move_point("U", knot),
            (2, 0) => move_point("R", knot),
            (0, -2) => move_point("D", knot),
            (-2, 0) => move_point("L", knot),
    
            // The rest of this `match` means tail needs to move diagonally to catch up
    
            // UR
            (1, 2) | (2, 1) | (2, 2) => {
                move_point("U", knot);
                move_point("R", knot);
            },
    
            // UL
            (-1, 2) | (-2, 1) | (-2, 2) => {
                move_point("U", knot);
                move_point("L", knot);
            }
    
            // DL
            (-1, -2) | (-2, -1) | (-2, -2) => {
                move_point("D", knot);
                move_point("L", knot);
            },
    
            // DR
            (1, -2) | (2, -1) | (2, -2) => {
                move_point("D", knot);
                move_point("R", knot);
            },
    
            _ => panic!("Tail is too far away from head! (dx={}, dy={})", dx, dy)
        };

        prev = *knot;
    }
}


#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let answer = part1(input);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part2_sample_input() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let answer = part2(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn part2_larger_sample_input() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";

        let answer = part2(input);
        assert_eq!(answer, 36);
    }

    /**
     * . . . . .
     * . . # . .
     * . # # # .
     * . . # . .
     * . . . . .
     */
    #[test]
    fn test_plus() {
        let input = "U 2\nD 2\nR 2\nL 2\nD 2\nU 2\nL 2\nR 2";

        let answer = part1(input);
        assert_eq!(answer, 5);
    }

    /**
     * . . . . .
     * . # . # .
     * . . # . .
     * . # . # .
     * . . . . .
     */
    #[test]
    fn test_cross() {
        let input = "U 1\nR 1\nU 1";

        let answer = part1(input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn move_point_r() {
        let mut p: Point = (0, 0);
        move_point("R", &mut p);

        assert_eq!(p.0, 1);
        assert_eq!(p.1, 0);
    }
}