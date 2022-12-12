use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Direction {
    fn from_string(val: &str) -> Result<Self, &str> {
        match val {
            "U" => Ok(Self::UP),
            "R" => Ok(Self::RIGHT),
            "D" => Ok(Self::DOWN),
            "L" => Ok(Self::LEFT),
            _ => Err(&"Bad string value"),
        }
    }
}

struct Knot {
    x: i32,
    y: i32,
}

impl Clone for Knot {
    fn clone(&self) -> Self {
        Knot {
            x: self.x,
            y: self.y,
        }
    }
}

impl Knot {
    fn follow(&mut self, head: &Knot) {
        if (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1 {
            return;
        }

        if head.x > self.x {
            self.x += 1;
        } else if head.x < self.x {
            self.x -= 1;
        }

        if head.y > self.y {
            self.y += 1;
        } else if head.y < self.y {
            self.y -= 1;
        }
    }

    fn move_in_direction(&mut self, dir: &Direction) {
        match dir {
            Direction::UP => self.y += 1,
            Direction::RIGHT => self.x += 1,
            Direction::DOWN => self.y -= 1,
            Direction::LEFT => self.x -= 1,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let mut head = Knot { x: 0, y: 0 };
    let mut tail = Knot { x: 0, y: 0 };
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let direction = Direction::from_string(tokens[0]).unwrap();
        let step = tokens[1].parse::<u32>().unwrap();

        for i in 0..step {
            head.move_in_direction(&direction);
            tail.follow(&head);

            visited.insert((tail.x, tail.y));
        }

        println!(
            "Head at {},{}, Tail at {}, {}",
            head.x, head.y, tail.x, tail.y
        );
    }

    println!("Visited {} positions", visited.len());
}

fn part2(contents: &String) {
    let mut knots: Vec<Knot> = Vec::new();
    for _i in 0..10 {
        knots.push(Knot { x: 0, y: 0 });
    }
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for line in contents.lines() {
        let tokens: Vec<&str> = line.split(" ").collect();
        let direction = Direction::from_string(tokens[0]).unwrap();
        let step = tokens[1].parse::<u32>().unwrap();

        for _i in 0..step {
            knots[0].move_in_direction(&direction);

            for j in 0..knots.len() - 1 {
                let leader = knots[j].clone();
                let follower = &mut knots[j + 1];
                follower.follow(&leader);
            }

            visited.insert((knots[9].x, knots[9].y));
        }
    }

    println!("Visited {} positions", visited.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);

        let test_contents = String::from_str(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
