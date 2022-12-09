use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {}

fn part2(contents: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
