use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

struct World {
    beacons: HashSet<(i32, i32)>,
    sensors: HashMap<(i32, i32), i32>, // Sensors with their viewing distance
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl World {
    fn new() -> Self {
        World {
            beacons: HashSet::new(),
            sensors: HashMap::new(),
            min_x: std::i32::MAX,
            max_x: 0,
            min_y: std::i32::MAX,
            max_y: 0,
        }
    }

    fn add_result(&mut self, line: &str) {
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();

        let cap = re.captures_iter(line).next().unwrap();
        let sensor_coord = (
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        );
        let beacon_coord = (
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        );
        let view_dist = manhattan_dist(sensor_coord, beacon_coord);
        self.sensors.insert(sensor_coord, view_dist);
        self.beacons.insert(beacon_coord);

        if sensor_coord.0 - view_dist < self.min_x {
            self.min_x = sensor_coord.0 - view_dist
        }
        if sensor_coord.0 + view_dist > self.max_x {
            self.max_x = sensor_coord.0 + view_dist
        }
        if sensor_coord.1 < self.min_y {
            self.min_y = sensor_coord.1
        }
        if sensor_coord.1 > self.max_y {
            self.max_y = sensor_coord.1
        }

        if beacon_coord.0 < self.min_x {
            self.min_x = beacon_coord.0
        }
        if beacon_coord.0 > self.max_x {
            self.max_x = beacon_coord.0
        }
        if beacon_coord.1 < self.min_y {
            self.min_y = beacon_coord.1
        }
        if beacon_coord.1 > self.max_y {
            self.max_y = beacon_coord.1
        }
    }

    fn scanned_pos(&self, y: i32) -> u32 {
        let mut pos = 0;

        for x in self.min_x..=self.max_x {
            let mut scanned = false;
            for (sensor_coord, view_dist) in self.sensors.iter() {
                if self.beacons.get(&(x, y)).is_some() {
                    scanned = true;
                    // print!("B");
                    break;
                }
                if (x, y) == *sensor_coord {
                    scanned = true;
                    // print!("S");
                    break;
                }
                if manhattan_dist(*sensor_coord, (x, y)) <= *view_dist {
                    pos += 1;
                    scanned = true;
                    // print!("#");
                    break;
                }
            }
            // if !scanned {
            //     print!(".");
            // }
        }
        // println!("");

        pos
    }

    fn display(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.beacons.get(&(x, y)).is_some() {
                    print!("B");
                } else if self.sensors.get(&(x, y)).is_some() {
                    print!("S");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        println!("");
    }
}

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let mut world = World::new();

    for line in contents.lines() {
        world.add_result(line);
    }

    // world.display();

    println!("\nline 10 {}", world.scanned_pos(10));
    println!("\nline 11 {}", world.scanned_pos(11));
    println!("\nline 2000000 {}", world.scanned_pos(2000000));
}

fn part2(contents: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
