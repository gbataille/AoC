use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::rc::Rc;
use std::vec::Vec;
use utils::graph;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Coordinates {
    x: i64,
    y: i64,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Node {
    coord: Coordinates,
    height: i64,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.coord.x, self.coord.y))
    }
}

#[derive(Debug, Clone)]
struct Graph {
    nodes: HashMap<Coordinates, Rc<Node>>,
    height: i64,
    width: i64,
}

impl graph::Graphable for Graph {
    type Node = Node;
    type Coordinates = Coordinates;
    type PathWeight = i64;

    fn coords_for(&self, node: &Self::Node) -> Self::Coordinates {
        node.coord.clone()
    }

    fn node_at(&self, coords: &Self::Coordinates) -> Option<Rc<Self::Node>> {
        match self.nodes.get(coords) {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }

    fn neighbours(&self, node: &Node) -> Vec<(Rc<Node>, i64)> {
        let mut accessible_neighbours = Vec::new();

        for shift in [(0i64, 1i64), (1, 0), (0, -1), (-1, 0)] {
            let neighbour_coord = Coordinates {
                x: node.coord.x + shift.0,
                y: node.coord.y + shift.1,
            };
            // Exclude borders
            if neighbour_coord.x < 0
                || neighbour_coord.y < 0
                || neighbour_coord.x >= self.width
                || neighbour_coord.y >= self.height
            {
                continue;
            }

            let neighbour = self.node_at(&neighbour_coord).clone().unwrap();
            if neighbour.height <= node.height + 1 {
                accessible_neighbours.push((neighbour, 1));
            }
        }

        accessible_neighbours
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

    let mut pf = graph::PathFinder::new(&start, &end, &graph).unwrap();
    let soluce = pf.find_shortest_dist();
    match soluce {
        None => println!("No solution"),
        Some(min_path) => println!("Part 1's solution is {}", &min_path),
    }
}

fn part2(contents: &String) {
    let (map, _start, end) = to_map(contents);
    let graph = build_graph(&map);

    let mut best = std::i64::MAX;

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            if *height == 10 {
                let start = Coordinates {
                    x: x as i64,
                    y: y as i64,
                };

                let mut pf = graph::PathFinder::new(&start, &end, &graph).unwrap();
                let soluce = pf.find_shortest_dist();
                match soluce {
                    None => (),
                    Some(min_path) => {
                        if min_path < best {
                            best = min_path;
                        }
                    }
                }
            }
        }
    }

    println!("\nPart 2's best solution is {}", &best);
}

fn to_map(contents: &String) -> (Vec<Vec<i64>>, Coordinates, Coordinates) {
    let mut map: Vec<Vec<i64>> = Vec::new();
    let mut start = Coordinates { x: -1, y: -1 };
    let mut end = Coordinates { x: -1, y: -1 };
    for (y, line) in contents.lines().enumerate() {
        let mut line_vec: Vec<i64> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let val = match c {
                'S' => {
                    start = Coordinates {
                        x: x as i64,
                        y: y as i64,
                    };
                    10
                }
                'E' => {
                    end = Coordinates {
                        x: x as i64,
                        y: y as i64,
                    };
                    35
                }
                letter => letter.to_digit(36).unwrap() as i64,
            };

            line_vec.push(val);
        }

        map.push(line_vec);
    }

    (map, start, end)
}

fn build_graph(map: &Vec<Vec<i64>>) -> Graph {
    let mut nodes = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        for (x, height) in line.iter().enumerate() {
            let coord = Coordinates {
                x: x as i64,
                y: y as i64,
            };

            nodes.insert(
                coord.clone(),
                Rc::new(Node {
                    coord: coord.clone(),
                    height: *height as i64,
                }),
            );
        }
    }

    Graph {
        nodes,
        height: map.len() as i64,
        width: map[0].len() as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

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
