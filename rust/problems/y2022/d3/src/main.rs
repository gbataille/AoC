use std::fs;

fn main() {
    part1();
    part2();
}

fn score(letter: char) -> u32 {
    let ascii = letter as u32;
    if ascii < 97 {
        ascii - 65 + 27
    } else {
        ascii - 97 + 1
    }
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw";

    let mut res: u32 = 0;

    for line in contents.split("\n") {
        let one = &line[..line.len() / 2];
        let two = &line[(line.len() / 2)..];

        for letter in one.chars() {
            if two.contains(&letter.to_string()) {
                let s = score(letter);
                println!("{} - {} - {}", line, letter, s);
                res += s;
                break;
            }
        }
    }

    println!("{res}");
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    // let contents = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw";
    let mut res: u32 = 0;
    let mut it = contents.split("\n");
    let mut one = it.next().unwrap();
    let mut two = it.next().unwrap();
    let mut three = it.next().unwrap();
    loop {
        for letter in one.chars() {
            if two.contains(&letter.to_string()) && three.contains(&letter.to_string()) {
                let s = score(letter);
                res += s;
                break;
            }
        }
        one = it.next().unwrap_or("");
        if one == "" {
            break;
        }
        two = it.next().unwrap();
        three = it.next().unwrap();
    }

    println!("{res}");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    fn not_a_test() {
        assert_eq!(1, 2);
    }

    #[test]
    fn test_add3() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_add5() {
        assert_eq!(add(3, 2), 5);
    }

    fn foo() {
        println!("{}", 'a' as u32);
    }

    #[test]
    fn test_ascii() {
        println!("{}", 'a' as u32);
        println!("{}", 'A' as u32);
    }
    #[test]
    fn bar() {
        println!("{}", 'a' as u32);
        println!("{}", 'A' as u32);
    }
}

mod foo {
    fn foobar() {
        println!("{}", 'a' as u32);
        println!("{}", 'A' as u32);
    }
}
