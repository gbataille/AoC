use std::fs;
use std::vec::Vec;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut max = 0;
    let mut current = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            if current > max {
                max = current
            }
            current = 0;
            continue;
        }
        current += line.parse::<i32>().unwrap();
    }

    println!("{max}")
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut calories = Vec::new();
    let mut current = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            calories.push(current);
            current = 0;
            continue;
        }
        current += line.parse::<i32>().unwrap();
    }

    calories.sort();
    calories.reverse();

    let mut tot = 0;
    for i in [0, 1, 2] {
        tot += calories[i]
    }

    println!("{tot}")
}
