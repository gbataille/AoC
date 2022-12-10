// COMMUNICATION SYSTEM
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    println!("{}", find_unique_window(contents, 4));
}

fn is_unique_window(window: &str) -> bool {
    let mut unique = HashMap::new();
    for elem in window.chars() {
        if unique.contains_key(&elem) {
            return false;
        } else {
            unique.insert(elem, true);
        }
    }
    return true;
}

fn part2(contents: &String) {
    println!("{}", find_unique_window(contents, 14));
}

fn find_unique_window(data: &String, length: usize) -> usize {
    for i in 0..(data.len() - length) {
        let window = &data[i..(i + length)];
        if is_unique_window(window) {
            return i + length;
        }
    }
    return usize::MAX;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap();
        println!("{}", test_contents);
        part1(&test_contents);
        part2(&test_contents);
        println!("-----");
        let test_contents = String::from_str("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap();
        println!("{}", test_contents);
        part1(&test_contents);
        part2(&test_contents);
        println!("-----");
        let test_contents = String::from_str("nppdvjthqldpwncqszvftbrmjlhg").unwrap();
        println!("{}", test_contents);
        part1(&test_contents);
        part2(&test_contents);
        println!("-----");
        let test_contents = String::from_str("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap();
        println!("{}", test_contents);
        part1(&test_contents);
        part2(&test_contents);
        println!("-----");
        let test_contents = String::from_str("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap();
        println!("{}", test_contents);
        part1(&test_contents);
        part2(&test_contents);
        println!("-----");
    }
}
