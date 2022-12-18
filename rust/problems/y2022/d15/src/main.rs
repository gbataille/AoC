use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
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

    fn nb_beacons_and_sensors_at(&self, y: i32) -> i32 {
        let mut tot = 0;
        for beacon in self.beacons.iter() {
            if beacon.1 == y {
                tot += 1;
            }
        }
        for (sensor, _) in self.sensors.iter() {
            if sensor.1 == y {
                tot += 1;
            }
        }
        tot
    }

    fn scanned_pos_with_ranges(&self, y: i32, clip: bool) -> VecDeque<(i32, i32)> {
        let min_boundary = 0;
        let max_boundary = 20;
        let mut ranges: VecDeque<(i32, i32)> = VecDeque::new();

        for (i, (sensor, sensor_view)) in self.sensors.iter().enumerate() {
            // println!(
            //     "\n================\nRunning sensor {} at {:?}, with view {}, for line {}",
            //     i, sensor, sensor_view, y
            // );
            let sensor_range = sensor_range_for_line(&sensor, *sensor_view, y);
            // println!("Sees {:?} at line {}", sensor_range, y);
            match sensor_range {
                None => continue,
                Some(range) => {
                    let clipped: (i32, i32);
                    if clip {
                        clipped = clip_range(range, min_boundary, max_boundary);
                    } else {
                        clipped = range;
                    }
                    if ranges.len() == 0 {
                        ranges.push_back(clipped);
                        continue;
                    }

                    let mut new_ranges: VecDeque<(i32, i32)> = VecDeque::new();
                    let mut cur_range = Some(clipped);
                    for r in ranges.into_iter() {
                        match cur_range {
                            None => new_ranges.push_back(r),
                            Some(range) => {
                                // println!("Comparing {:?} and {:?}", &range, &r);
                                if are_overlapping(range, r) {
                                    let merged = merge(range, r);
                                    // println!("Overlapping, result in {:?}", &merged);
                                    cur_range = Some(merged);
                                } else {
                                    if r.0 < range.0 {
                                        // println!(
                                        //     "Non verlapping, pushing {:?} keeping {:?}",
                                        //     &r, &range
                                        // );
                                        new_ranges.push_back(r);
                                        // we continue to try and fit our new range in the ranges
                                        // list
                                    } else {
                                        // println!(
                                        //     "Non verlapping, pushing {:?} then {:?}",
                                        //     &range, &r
                                        // );
                                        new_ranges.push_back(range);
                                        new_ranges.push_back(r);
                                        cur_range = None;
                                        // the new range is smaller than the last checked and non
                                        // overlapping, it goes in the range list as is
                                    }
                                }
                            }
                        }
                    }
                    match cur_range {
                        None => (),
                        Some(a) => new_ranges.push_back(a),
                    }
                    ranges = new_ranges;
                }
            }
        }

        ranges
    }

    fn scanned_pos_slow(&self, y: i32) -> u32 {
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

fn sensor_range_for_line(sensor_coord: &(i32, i32), view_dist: i32, y: i32) -> Option<(i32, i32)> {
    let y_dist = (y - sensor_coord.1).abs();
    if y_dist > view_dist {
        return None;
    }

    return Some((
        sensor_coord.0 - view_dist + y_dist,
        sensor_coord.0 + view_dist - y_dist,
    ));
}

fn clip_range(range: (i32, i32), min: i32, max: i32) -> (i32, i32) {
    let mut from = range.0;
    let mut to = range.1;

    if from < min {
        from = min;
    }
    if to > max {
        to = max;
    }

    (from, to)
}

fn are_overlapping(a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 <= b.1 && a.0 >= b.0) || (a.1 >= b.0 && a.1 <= b.1)
}

fn merge(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let mut from = a.0;
    let mut to = a.1;
    if b.0 < from {
        from = b.0;
    }
    if b.1 > to {
        to = b.1;
    }
    (from, to)
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
    for y in [10, 11, 2000000] {
        let ranges = world.scanned_pos_with_ranges(y, false);
        println!("\nline {}", y);
        let mut tot = 0;
        for r in ranges.iter() {
            tot += r.1 - r.0 + 1;
        }
        tot -= world.nb_beacons_and_sensors_at(y);
        println!("scanned lines {}", tot);
    }
}

fn part2(contents: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_range() {
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 0));
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 1));
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 8));
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 7));
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 7));
        println!("{:?}", sensor_range_for_line(&(2, 4), 5, 10));
    }

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
