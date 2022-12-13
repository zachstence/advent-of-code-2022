use std::{fmt::Debug, collections::VecDeque};

use itertools::Itertools;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {
    run(input, 20, true)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {
    run(input, 10_000, false)
}

fn run(input: &str, num_rounds: u32, div_by_3: bool) -> u64 {
    // Read monkeys
    let mut monkeys = input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples()
        .map(|(_, items_line, operation_line, test_line, test_true_line, test_false_line)| {
            let items = parse_items(items_line);
            let operation = parse_operation(operation_line);
            let (test_num, test) = parse_test(test_line, test_true_line, test_false_line);
            
            Monkey { items, operation, test, test_num, inspect_count: 0 }
        })
        .collect::<Vec<Monkey>>();
    let num_monkeys = monkeys.len();

    let lcm: u32 = monkeys.iter().map(|m| m.test_num).product();

    // Simulate monkeys
    for _ in 0..num_rounds {

        for m in 0..num_monkeys {
            let (monkey, mut rest) = get_rest_mut(&mut monkeys, m);
            let monkey = monkey.unwrap();

            while !monkey.items.is_empty() {
                let mut item = monkey.items.pop_front().unwrap();
                    
                monkey.inspect_count += 1;
                item = (monkey.operation)(item);
                if div_by_3 {
                    item /= 3;
                } else {
                    item %= lcm as u64;
                }
                let throw_to = (monkey.test)(item);
                
                let monkey_to_throw_to = rest.get_mut(throw_to).unwrap().as_mut().unwrap();
                monkey_to_throw_to.items.push_back(item);
            }
        }
    }

    monkeys.sort_by(|m1, m2| m2.inspect_count.cmp(&m1.inspect_count));

    let max1 = monkeys.get(0).unwrap();
    let max2 = monkeys.get(1).unwrap();

    max1.inspect_count * max2.inspect_count
}

fn get_rest_mut(v: & mut [Monkey], i: usize) -> (Option<& mut Monkey>, Vec<Option<&mut Monkey>>) {
    let (before, after) = v.split_at_mut(i);
    let (taken, after) = after.split_at_mut(1);
    let taken = taken.get_mut(0);

    let before_options = before.iter_mut().map(Some).collect::<Vec<Option<&mut Monkey>>>();
    let mut after_options = after.iter_mut().map(Some).collect::<Vec<Option<&mut Monkey>>>();
    
    let mut rest = before_options;
    rest.push(None);
    rest.append(&mut after_options);

    (taken, rest)
}


type OperationFn = Box<dyn Fn(u64) -> u64>;
type TestFn = Box<dyn Fn(u64) -> usize>;

fn parse_test(test_line: &str, test_true_line: &str, test_false_line: &str) -> (u32, TestFn) {
    let num = test_line.get(21..).unwrap().parse::<u32>().unwrap();
    let true_monkey = test_true_line.get(29..).unwrap().parse::<usize>().unwrap();
    let false_monkey = test_false_line.get(30..).unwrap().parse::<usize>().unwrap();

    (num, Box::new(move |item| if item % (num as u64) == 0 { true_monkey } else { false_monkey }))
}

fn parse_operation(line: &str) -> OperationFn {
    let op = line.get(23..24).unwrap();
    let num = line.get(25..).unwrap().parse::<u64>();

    match (op, num) {
        ("+", Ok(num)) => Box::new(move |item| item + num),
        ("*", Ok(num)) => Box::new(move |item| item * num),

        // If parsing u64 fails, value is "old" so its item
        ("+", Err(_)) => Box::new(move |item| item + item),
        ("*", Err(_)) => Box::new(move |item| item * item),
        _ => panic!("Unknown op {}", op),
    }
}

fn parse_items(line: &str) -> VecDeque<u64> {
    line[18..]
        .split(", ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect::<VecDeque<u64>>()
}

struct Monkey {
    items: VecDeque<u64>,
    operation: OperationFn,
    test: TestFn,
    test_num: u32,
    inspect_count: u64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspect_count", &self.inspect_count)
            .finish()
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    #[test]
    fn part1_sample_input() {
        let input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1";

        let answer = part1(input);
        assert_eq!(answer, 10605);
    }
}
