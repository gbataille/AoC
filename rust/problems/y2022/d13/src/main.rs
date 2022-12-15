use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Right,
    Wrong,
    Undecided,
}

#[derive(Debug)]
enum Elem {
    Integer(u32),
    List(Vec<Elem>),
}

impl fmt::Display for Elem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Elem::Integer(a) => {
                write!(f, "{}", a);
            }
            Elem::List(v) => {
                write!(f, "[");
                for item in v.iter() {
                    write!(f, "{}", item.to_string());
                    write!(f, ",");
                }
                write!(f, "]");
            }
        }
        Ok(())
    }
}

impl Elem {
    fn is_integer(&self) -> bool {
        match self {
            Elem::Integer(_) => true,
            _ => false,
        }
    }

    fn is_list(&self) -> bool {
        !self.is_integer()
    }

    fn to_int(&self) -> Option<u32> {
        match self {
            Elem::Integer(a) => Some(*a),
            _ => None,
        }
    }

    fn to_list(&self) -> Option<&Vec<Elem>> {
        match self {
            Elem::List(a) => Some(a),
            _ => None,
        }
    }

    fn parse(contents: &str) -> Self {
        let mut queue: Vec<Vec<Elem>> = Vec::new();
        let mut current_vec: Vec<Elem> = Vec::new();
        // processing of numbers is super ugly but seems to work
        let mut current_num: Vec<char> = Vec::new();

        // remove the surrounding []
        for c in contents[1..contents.len() - 1].chars() {
            match c {
                '[' => {
                    queue.push(current_vec);
                    current_vec = Vec::new();
                }
                ']' => {
                    if current_num.len() > 0 {
                        current_vec.push(Elem::Integer(
                            current_num
                                .into_iter()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap(),
                        ));
                        current_num = Vec::new();
                    }

                    let mut parent = queue.pop().unwrap();
                    parent.push(Elem::List(current_vec));
                    current_vec = parent;
                }
                ',' => {
                    if current_num.len() > 0 {
                        current_vec.push(Elem::Integer(
                            current_num
                                .into_iter()
                                .collect::<String>()
                                .parse::<u32>()
                                .unwrap(),
                        ));
                        current_num = Vec::new();
                    }
                }
                digit => {
                    current_num.push(digit);
                }
            }
        }
        // Process the last entry that had been excluded by truncating the surrounding []
        if current_num.len() > 0 {
            current_vec.push(Elem::Integer(
                current_num
                    .into_iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap(),
            ));
        }

        Self::List(current_vec)
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut line_iter = lines.iter();
    let mut idx = 1;
    let mut score = 0;

    loop {
        // println!("\n\n###############\nPackets {}", idx);
        let left_packet = line_iter.next().unwrap();
        let right_packet = line_iter.next().unwrap();

        let out = good_packet_order(&Elem::parse(&left_packet), &Elem::parse(&right_packet));
        // println!("Packets {} - {:?}", idx, out);

        if out == Outcome::Right {
            score += idx;
        }

        let blank = line_iter.next();
        if blank.is_none() {
            break;
        }

        idx += 1;
    }

    println!("Final score {}", score);
}

fn part2(contents: &String) {
    let mut lines: Vec<Elem> = Vec::new();
    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }
        lines.push(Elem::parse(line));
    }

    lines.push(Elem::List(vec![Elem::List(vec![Elem::Integer(2)])]));
    lines.push(Elem::List(vec![Elem::List(vec![Elem::Integer(6)])]));

    lines.sort_by(|a, b| {
        if good_packet_order(a, b) == Outcome::Right {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let mut decoder = 1;
    let mut idx = 1;
    for line in lines.iter() {
        let repr = line.to_string();
        if repr == "[[2,],]" || repr == "[[6,],]" {
            decoder *= idx;
        }
        println!("{}", repr);

        idx += 1;
    }

    println!("Decoder value: {}", decoder);
}

fn good_packet_order(left: &Elem, right: &Elem) -> Outcome {
    // println!("Comparing {:?} and {:?}", left, right);
    if left.is_integer() && right.is_integer() {
        let l = left.to_int().unwrap();
        let r = right.to_int().unwrap();
        if l == r {
            return Outcome::Undecided;
        } else if l < r {
            return Outcome::Right;
        } else {
            return Outcome::Wrong;
        }
    }

    if left.is_list() && right.is_list() {
        let mut left_iter = left.to_list().unwrap().iter();
        let mut right_iter = right.to_list().unwrap().iter();

        loop {
            let n_left = left_iter.next();
            let n_right = right_iter.next();

            if n_left.is_none() && n_right.is_none() {
                return Outcome::Undecided;
            } else if n_left.is_none() && !n_right.is_none() {
                return Outcome::Right;
            } else if !n_left.is_none() && n_right.is_none() {
                return Outcome::Wrong;
            } else {
                let out = good_packet_order(n_left.unwrap(), n_right.unwrap());
                if out == Outcome::Undecided {
                    continue;
                } else {
                    return out;
                }
            }
        }
    }

    if left.is_list() && right.is_integer() {
        return good_packet_order(
            left,
            &Elem::List(vec![Elem::Integer(right.to_int().unwrap())]),
        );
    } else {
        return good_packet_order(
            &Elem::List(vec![Elem::Integer(left.to_int().unwrap())]),
            right,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_Elem_parse() {
        dbg!(Elem::parse(&"[[[]]]"));
    }

    #[test]
    fn test_1() {
        let test_contents = String::from_str(
            "[1,1,3,1,1]
[1,1,5,1,1]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_2() {
        let test_contents = String::from_str(
            "[[1],[2,3,4]]
[[1],4]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_3() {
        let test_contents = String::from_str(
            "[9]
[[8,7,6]]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_4() {
        let test_contents = String::from_str(
            "[[4,4],4,4]
[[4,4],4,4,4]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_5() {
        let test_contents = String::from_str(
            "[7,7,7,7]
[7,7,7]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_6() {
        let test_contents = String::from_str(
            "[]
[3]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_7() {
        let test_contents = String::from_str(
            "[[[]]]
[[]]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_8() {
        let test_contents = String::from_str(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
        .unwrap();
        part1(&test_contents);
    }

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
