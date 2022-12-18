use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn print_dbg(msg: &str) {
    if env::var("DEBUG").is_ok() {
        print!("{}", msg);
    }
}

fn println_dbg(msg: &str) {
    if env::var("DEBUG").is_ok() {
        println!("{}", msg);
    }
}

fn dbg_dbg(data: &dyn Debug) {
    if env::var("DEBUG").is_ok() {
        dbg!(data);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    println_dbg(contents);
}

fn part2(contents: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        env::set_var("DEBUG", "true");

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
