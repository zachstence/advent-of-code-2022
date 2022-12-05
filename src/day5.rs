use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref STACK_REGEX: Regex = Regex::new(r".(.). ?").unwrap();
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let (initial_state_str, moves_str) = input.split_once("\n\n").unwrap();
    let initial_state = initial_state_str.lines();
    let moves = moves_str.lines();
    
    // Set up stacks initial state
    let mut stacks: Vec<Vec<&str>> = vec![];

    for (l, line) in initial_state.rev().enumerate() {
        // Initialize stacks from first line
        if l == 0 {
            let num_stacks = (line.len() + 1) / 4;
            stacks = vec![vec![] as Vec<&str>; num_stacks];
            continue;
        }

        let caps = STACK_REGEX.captures_iter(line)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect::<Vec<&str>>();
        
        for (c, cap) in caps.iter().enumerate() {
            if cap == &" " { continue; }
            stacks.get_mut(c).unwrap().push(cap);
        }
    }

    // Perform moves
    for line in moves {
        let Move {num_crates, from_index, to_index} = parse_move(line);

        #[allow(clippy::single_element_loop)]
        for _ in 0..num_crates {
            let crate_to_move = stacks.get_mut(from_index).unwrap().pop().unwrap();
            stacks.get_mut(to_index).unwrap().push(crate_to_move);
        }
    }

    // Get message containing top item on each stack
    stacks.into_iter().map(|stack| stack.last().copied().unwrap()).collect::<Vec<&str>>().join("")
}


struct Move {
    num_crates: u32,
    from_index: usize,
    to_index: usize,
}

fn parse_move(line: &str) -> Move {
    let tokens = line.split_whitespace().collect::<Vec<&str>>();

    Move {
        num_crates: tokens.get(1).unwrap().parse::<u32>().unwrap(),
        from_index: tokens.get(3).unwrap().parse::<usize>().unwrap() - 1,
        to_index: tokens.get(5).unwrap().parse::<usize>().unwrap() - 1,
    }
}