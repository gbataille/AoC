use std::fs;
use std::vec::Vec;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut overlap = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        if line_has_full_overlap(&line) {
            overlap += 1
        }
    }

    println!("{}", overlap);
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut overlap = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        if line_has_any_overlap(&line) {
            overlap += 1
        }
    }

    println!("{}", overlap);
}

fn line_to_range_pair(line: &str) -> ([i32; 2], [i32; 2]) {
    let pairs: Vec<&str> = line.split(",").collect();
    if pairs.len() != 2 {
        panic!("not having 2 pairs - {}", line);
    }
    let first: Vec<&str> = pairs[0].split("-").collect();
    let second: Vec<&str> = pairs[1].split("-").collect();

    let first_range: [i32; 2] = [
        first[0].parse::<i32>().unwrap(),
        first[1].parse::<i32>().unwrap(),
    ];
    let second_range: [i32; 2] = [
        second[0].parse::<i32>().unwrap(),
        second[1].parse::<i32>().unwrap(),
    ];

    return (first_range, second_range);
}

fn line_has_full_overlap(line: &str) -> bool {
    let (first_range, second_range) = line_to_range_pair(line);

    if (first_range[0] <= second_range[0] && first_range[1] >= second_range[1])
        || (first_range[0] >= second_range[0] && first_range[1] <= second_range[1])
    {
        return true;
    }
    return false;
}

fn line_has_any_overlap(line: &str) -> bool {
    let (first_range, second_range) = line_to_range_pair(line);

    if (first_range[0] >= second_range[0] && first_range[0] <= second_range[1])
        || (first_range[1] >= second_range[0] && first_range[1] <= second_range[1])
        || (second_range[0] >= first_range[0] && second_range[0] <= first_range[1])
        || (second_range[1] >= first_range[0] && second_range[1] <= first_range[1])
    {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        println!("{}", 'a' as u32);
        println!("{}", 'A' as u32);
    }
}
