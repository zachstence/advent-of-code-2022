use std::{str::FromStr, iter::zip, cmp::Ordering, fmt::Display};

use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    // Add newlines so .tuples() reads the last pair of packets
    let input = String::from(input) + "\n\n";

    let cmps = input
        .lines()
        .tuples()
        .map(|(left, right, _)| {
            let (left_parsed, right_parsed) = (left.parse::<Value>().unwrap(), right.parse::<Value>().unwrap());
            left_parsed.cmp(&right_parsed)
        })
        .collect::<Vec<_>>();

    cmps
        .iter()
        .enumerate()
        .map(|(i, res)| if *res == Ordering::Less { i + 1 } else { 0 })
        .sum()
}

// #[aoc(day13, part2)]
// pub fn part2(input: &str) -> u32 {
//     0
// }


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
    List(Vec<Value>),
    Integer(i32),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::List(list) => write!(f, "[{}]", list.iter().map(|v| format!("{v}")).join(",")),
            Value::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl FromStr for Value {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str::<Value>(s)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other { return Ordering::Equal };

        match (self, other) {
            // Compare Integer Values by comparing the integers
            (Value::Integer(s), Value::Integer(o)) => s.cmp(o),
            // Compare List Values by comparing each element
            (Value::List(s), Value::List(o)) => {
                // If the lists have an element that differs, sort by that element
                let mismatch = zip(s, o).find(|(sv, ov)| sv != ov);
                if let Some((sv, ov)) = mismatch {
                    return sv.cmp(ov);
                }
                
                // Otherwise, the lists must have different lengths, sort by their lengths
                s.len().cmp(&o.len())
            },
            // If one value is a List and the other is an Integer, convert the Integer to a List and compare
            (Value::Integer(s), Value::List(_)) => Value::List(vec![Value::Integer(*s)]).cmp(other),
            (Value::List(_), Value::Integer(o)) => self.cmp(&Value::List(vec![Value::Integer(*o)])),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // Integer Values are equal if the integers are equal
            (Value::Integer(s), Value::Integer(o)) => s.eq(o),
            // List Values are equal if they have the same length and each element is equal
            (Value::List(s), Value::List(o)) => s.len() == o.len() && zip(s, o).fold(true, |acc, (si, oi)| acc && si.eq(oi)),
            // List and Integer Values are never equal
            (Value::Integer(s), Value::List(_)) => Value::List(vec![Value::Integer(*s)]).eq(other),
            (Value::List(_), Value::Integer(o)) => self.eq(&Value::List(vec![Value::Integer(*o)])),
        }
    }
}

impl Eq for Value { }

#[cfg(test)]
mod day13_tests {
    use super::*;

    const SAMPLE_INPUT: &str = concat!(
        "[1,1,3,1,1]\n",
        "[1,1,5,1,1]\n",
        "\n",
        "[[1],[2,3,4]]\n",
        "[[1],4]\n",
        "\n",
        "[9]\n",
        "[[8,7,6]]\n",
        "\n",
        "[[4,4],4,4]\n",
        "[[4,4],4,4,4]\n",
        "\n",
        "[7,7,7,7]\n",
        "[7,7,7]\n",
        "\n",
        "[]\n",
        "[3]\n",
        "\n",
        "[[[]]]\n",
        "[[]]\n",
        "\n",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]\n",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]\n",
    );

    #[test]
    fn part1_sample_input() {
        let answer = part1(SAMPLE_INPUT);
        assert_eq!(answer, 13);
    }

    // #[test]
    // fn part2_sample_input() {
    //     let answer = part2(SAMPLE_INPUT);
    //     assert_eq!(answer, 0);
    // }

    ////////
    
    #[test]
    fn value_from_str_integer() {
        let s = "5";
        let expected = Value::Integer(5);
        let actual = s.parse::<Value>().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn value_from_str_list() {
        let s = "[4,5,6]";
        let expected = Value::List(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]);
        let actual = s.parse::<Value>().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn value_from_str_nested_list() {
        let s = "[4,[5,5],6]";
        let expected = Value::List(vec![Value::Integer(4), Value::List(vec![Value::Integer(5), Value::Integer(5)]), Value::Integer(6)]);
        let actual = s.parse::<Value>().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn value_from_str_empty_list() {
        let s = "[[]]";
        let expected = Value::List(vec![Value::List(vec![])]);
        let actual = s.parse::<Value>().unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn value_eq() {
        let int1_a = Value::Integer(1);
        let int1_b = Value::Integer(1);
        let int2 = Value::Integer(2);
        let list_empty_a = Value::List(vec![]);
        let list_empty_b = Value::List(vec![]);
        let list_123_a = Value::List(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let list_123_b = Value::List(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let list_456 = Value::List(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]);

        let int1s_equal = int1_a == int1_b;
        assert!(int1s_equal);

        let int1_and_int2_not_equal = int1_a != int2;
        assert!(int1_and_int2_not_equal);

        let both_empty_equal = list_empty_a == list_empty_b;
        assert!(both_empty_equal);

        let empty_not_equal_to_123 = list_empty_a != list_123_a;
        assert!(empty_not_equal_to_123);

        let both_123_equal = list_123_a == list_123_b;
        assert!(both_123_equal);

        let list_123_and_456_not_equal = list_123_b != list_456;
        assert!(list_123_and_456_not_equal);

        let integer_and_list_not_equal = int1_b != list_empty_a && int2 != list_456;
        assert!(integer_and_list_not_equal);
    }

    #[test]
    fn value_cmp() {
        let int1 = Value::Integer(1);
        let int2 = Value::Integer(2);
        let list_empty = Value::List(vec![]);
        let list_123 = Value::List(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]);
        let list_456 = Value::List(vec![Value::Integer(4), Value::Integer(5), Value::Integer(6)]);
        let list_124 = Value::List(vec![Value::Integer(1), Value::Integer(2), Value::Integer(4)]);

        let int1s_equal = int1.cmp(&int1) == Ordering::Equal;
        assert!(int1s_equal);

        let both_empty_equal = list_empty.cmp(&list_empty) == Ordering::Equal;
        assert!(both_empty_equal);

        let both_123_equal = list_123.cmp(&list_123) == Ordering::Equal;
        assert!(both_123_equal);

        let lower_int_is_less = int1.cmp(&int2) == Ordering::Less;
        assert!(lower_int_is_less);

        let shorter_list_is_less = list_empty.cmp(&list_123) == Ordering::Less;
        assert!(shorter_list_is_less);

        let list_with_bigger_number_is_greater = list_456.cmp(&list_123) == Ordering::Greater;
        assert!(list_with_bigger_number_is_greater);

        let integer_is_less_then_list_with_multiple_elements = int1.cmp(&list_123) == Ordering::Less;
        assert!(integer_is_less_then_list_with_multiple_elements);

        let integer_is_greater_than_empty_list = int2.cmp(&list_empty) == Ordering::Greater;
        assert!(integer_is_greater_than_empty_list);

        let list_with_bigger_number_is_greater = list_124.cmp(&list_123) == Ordering::Greater;
        assert!(list_with_bigger_number_is_greater);
    }
}
