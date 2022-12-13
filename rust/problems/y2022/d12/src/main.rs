use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    coord: (i32, i32),
    height: i32,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<(i32, i32), Rc<Node>>,
    height: i32,
    width: i32,
}

impl Node {
    fn accessible_neighbours(&self, graph: &Graph) -> Vec<Rc<Node>> {
        let mut accessible_neighbours = Vec::new();

        for shift in [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)] {
            let neighbour_coord = (self.coord.0 + shift.0, self.coord.1 + shift.1);
            // Exclude borders
            if neighbour_coord.0 < 0
                || neighbour_coord.1 < 0
                || neighbour_coord.0 >= graph.width
                || neighbour_coord.1 >= graph.height
            {
                continue;
            }

            let neighbour = graph.nodes[&neighbour_coord].clone();
            if neighbour.height <= self.height + 1 {
                accessible_neighbours.push(neighbour);
            }
        }

        accessible_neighbours
    }
}

#[derive(Debug, Default)]
struct Dijkstra {
    heads: VecDeque<(u32, Rc<Node>)>,
    seen: HashMap<Rc<Node>, u32>,
}

impl Dijkstra {
    fn solve(&mut self, start: &(i32, i32), end: &(i32, i32), graph: &Graph) -> Option<u32> {
        self.heads = VecDeque::new();
        let start_node = &graph.nodes[start];
        self.heads.push_front((0, start_node.clone()));
        self.seen = HashMap::new();
        self.seen.insert(start_node.clone(), 0);

        loop {
            let o_head = self.heads.pop_front();
            let head;
            match o_head {
                None => {
                    return None;
                }
                Some(h) => head = h,
            }
            let path_length = head.0 + 1;
            for neighbor in head.1.accessible_neighbours(&graph) {
                let neighbor_length = self.seen.get(&neighbor).unwrap_or(&std::u32::MAX);
                if path_length < *neighbor_length {
                    self.heads.push_back((path_length, neighbor.clone()));
                    self.seen.insert(neighbor.clone(), path_length);

                    if neighbor.coord == *end {
                        return Some(head.0 + 1);
                    }
                }
            }

            // println!(">>>>>>>>>");
            // dbg!(&self);
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {
    let (map, start, end) = to_map(contents);
    let graph = build_graph(&map);

    println!("Going from {:?} to {:?}", start, end);

    let mut d = Dijkstra::default();
    let soluce = d.solve(&start, &end, &graph);
    match soluce {
        None => println!("No solution"),
        Some(min_path) => println!("Solution is {}", &min_path),
    }
}

fn part2(contents: &String) {
    let (map, _start, end) = to_map(contents);
    let graph = build_graph(&map);

    let mut best = std::u32::MAX;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 10 {
                let start = (x as i32, y as i32);
                println!("Going from {:?} to {:?}", start, end);

                let mut d = Dijkstra::default();
                let soluce = d.solve(&start, &end, &graph);
                match soluce {
                    None => println!("No solution"),
                    Some(min_path) => {
                        println!("Solution is {}", &min_path);
                        if min_path < best {
                            best = min_path;
                        }
                    }
                }
            }
        }
    }

    println!("\n Best solution is {}", &best);
}

fn to_map(contents: &String) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let mut map: Vec<Vec<i32>> = Vec::new();
    let mut start: (i32, i32) = (-1, -1);
    let mut end: (i32, i32) = (-1, -1);
    for (y, line) in contents.lines().enumerate() {
        let mut line_vec: Vec<i32> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let val = match c {
                'S' => {
                    start = (x as i32, y as i32);
                    10
                }
                'E' => {
                    end = (x as i32, y as i32);
                    35
                }
                letter => letter.to_digit(36).unwrap() as i32,
            };

            line_vec.push(val);
        }

        map.push(line_vec);
    }

    (map, start, end)
}

fn build_graph(map: &Vec<Vec<i32>>) -> Graph {
    let mut nodes = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            let coord = (x as i32, y as i32);

            nodes.insert(
                coord,
                Rc::new(Node {
                    coord,
                    height: *height as i32,
                }),
            );
        }
    }

    Graph {
        nodes,
        height: map.len() as i32,
        width: map[0].len() as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_digit() {
        println!("{}", 'a'.to_digit(36).unwrap_or(0));
        println!("{}", 'z'.to_digit(36).unwrap_or(0));
    }

    #[test]
    fn test_parts() {
        let test_contents = String::from_str(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
