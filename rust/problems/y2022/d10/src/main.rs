use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

struct CPU {
    X: Vec<i32>,
    cycle: u32,
}

impl CPU {
    fn run(&mut self, instr: &str) {
        let tokens: Vec<&str> = instr.split(" ").collect();
        match tokens[0] {
            "noop" => self.noop(),
            "addx" => {
                let v = tokens[1].parse::<i32>().unwrap();
                self.addx(v);
            }
            _ => (),
        }
    }

    fn noop(&mut self) {
        self.cycle += 1;
        self.X.push(*self.X.last().unwrap());
    }

    fn addx(&mut self, value: i32) {
        self.cycle += 2;
        self.X.push(*self.X.last().unwrap());
        self.X.push(*self.X.last().unwrap() + value);
    }

    fn value_at_cycle(&self, cycle: usize) -> i32 {
        let v: i32;
        if cycle >= self.X.len() {
            v = *self.X.last().unwrap();
        } else {
            v = self.X[cycle - 1];
        }
        v
    }

    fn signal_at_cycle(&self, cycle: usize) -> i32 {
        self.value_at_cycle(cycle) * (cycle as i32)
    }
}

fn part1(contents: &String) {
    let mut cpu = CPU {
        X: [1].to_vec(),
        cycle: 0,
    };

    for line in contents.lines() {
        cpu.run(line);
    }

    let mut sum = 0;
    for cycle in [20, 60, 100, 140, 180, 220] {
        let signal = cpu.signal_at_cycle(cycle as usize);
        println!("Signal strength at cycle {}: {}", cycle, signal);
        sum += signal;
    }

    println!("Sum {}", sum);
}

fn part2(contents: &String) {
    let mut cpu = CPU {
        X: [1].to_vec(),
        cycle: 0,
    };

    for line in contents.lines() {
        cpu.run(line);
    }

    let mut pixels: Vec<char> = Vec::new();
    for cycle in 1i32..=240 {
        let x = cpu.value_at_cycle(cycle as usize);
        if x + 1 >= (cycle - 1) % 40 && x - 1 <= (cycle - 1) % 40 {
            pixels.push('#');
        } else {
            pixels.push('.');
        }
    }

    println!("{:?}", String::from_iter(&pixels[0..40]));
    println!("{:?}", String::from_iter(&pixels[40..80]));
    println!("{:?}", String::from_iter(&pixels[80..120]));
    println!("{:?}", String::from_iter(&pixels[120..160]));
    println!("{:?}", String::from_iter(&pixels[160..200]));
    println!("{:?}", String::from_iter(&pixels[200..240]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
