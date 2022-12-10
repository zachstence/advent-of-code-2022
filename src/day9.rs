
use std::collections::HashSet;

type Point = (i32, i32);

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut head: Point = (0, 0);
    let mut tail: Point = (0, 0);

    visited.insert(tail);

    input
        .lines()
        .for_each(|line| {
            let (direction, distance) = line.split_once(' ').expect("Line should contain direction and distance");
            let distance = distance.parse::<i32>().expect("Distance should be an unsigned integer");

            #[allow(clippy::single_element_loop)]
            for _ in 0..distance {
                update_positions(direction, &mut head, &mut tail);
                visited.insert(tail);
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

fn update_positions(direction: &str, head: &mut Point, tail: &mut Point) {
    // First move head
    move_point(direction, head);

    // Move tail to catch up
    let (dx, dy) = delta(head, tail);

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
        // To catch up, we move the same direction that head moved
        (0, 2) | (0, -2) | (2, 0) | (-2, 0) => move_point(direction, tail),

        // The rest of this `match` means tail needs to move diagonally to catch up

        // UR
        (1, 2) | (2, 1) => {
            move_point("U", tail);
            move_point("R", tail);
        },

        // UL
        (-1, 2) | (-2, 1) => {
            move_point("U", tail);
            move_point("L", tail);
        }

        // DL
        (-1, -2) | (-2, -1) => {
            move_point("D", tail);
            move_point("L", tail);
        },

        // DR
        (1, -2) | (2, -1) => {
            move_point("D", tail);
            move_point("R", tail);
        },

        _ => panic!("Tail is too far away from head! (dx={}, dy={})", dx, dy)
    };

}


#[cfg(test)]
mod day9_tests {
    use super::*;

    #[test]
    fn sample_input() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";

        let answer = part1(input);
        assert_eq!(answer, 13);
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