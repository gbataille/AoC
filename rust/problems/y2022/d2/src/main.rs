use std::fs;

fn main() {
    part1();
    part2();
}

fn score(me: &str, other: &str) -> i32 {
    let score: i32;
    match me {
        "X" => {
            score = 1;
            match other {
                "A" => return score + 3,
                "B" => return score,
                "C" => return score + 6,
                _ => return 0,
            }
        }
        "Y" => {
            score = 2;
            match other {
                "A" => return score + 6,
                "B" => return score + 3,
                "C" => return score,
                _ => return 0,
            }
        }
        "Z" => {
            score = 3;
            match other {
                "A" => return score,
                "B" => return score + 6,
                "C" => return score + 3,
                _ => return 0,
            }
        }
        _ => return 0,
    }
}

fn compute_play(other: &str, instr: &str) -> &'static str {
    match other {
        "A" => match instr {
            "X" => return "Z",
            "Y" => return "X",
            "Z" => return "Y",
            _ => return "",
        },
        "B" => match instr {
            "X" => return "X",
            "Y" => return "Y",
            "Z" => return "Z",
            _ => return "",
        },
        "C" => match instr {
            "X" => return "Y",
            "Y" => return "Z",
            "Z" => return "X",
            _ => return "",
        },
        _ => return "",
    }
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut total = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut played = line.split(" ");
        let other = played.next().unwrap();
        let me = played.next().unwrap();
        total += score(&me, &other);
    }

    println!("{total}");
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let mut total = 0;
    for line in contents.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut played = line.split(" ");
        let other = played.next().unwrap();
        let me = played.next().unwrap();
        total += score(compute_play(&other, &me), &other);
    }

    println!("{total}");
}
