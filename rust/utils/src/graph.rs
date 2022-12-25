use super::debug;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::vec::Vec;

pub trait Graphable {
    type Node: Hash + Eq + Debug + Clone;
    type Coordinates: Eq + Debug + Clone;

    fn coords_for(&self, node: &Self::Node) -> Self::Coordinates;
    fn node_at(&self, coords: &Self::Coordinates) -> Option<Rc<Self::Node>>;
    fn neighbours(&self, node: &Self::Node) -> Vec<(Rc<Self::Node>, i64)>;
}

#[derive(Clone)]
pub struct Path<T: Graphable + Clone + Debug> {
    head: Rc<T::Node>,
    node_path: Vec<Rc<T::Node>>,
    nodes: HashMap<Rc<T::Node>, i64>,
    tot_length: i64,
}

impl<T: Graphable + Clone + Debug> Debug for Path<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Path ["))?;
        for node in self.node_path.iter() {
            let dist = self.nodes.get(node).unwrap();
            f.write_fmt(format_args!("{:?}:{},", node, dist))?;
        }
        f.write_fmt(format_args!("] - length {}", self.tot_length))
    }
}

impl<T: Graphable + Clone + Debug> Path<T> {
    fn grow(&mut self, node: Rc<T::Node>, path_length: i64) -> Result<(), ()> {
        if self.nodes.contains_key(&node) {
            return Err(());
        }
        self.tot_length += path_length;
        self.node_path.push(node.clone());
        self.nodes.insert(node.clone(), self.tot_length);
        self.head = node.clone();

        Ok(())
    }

    pub fn length(&self) -> i64 {
        self.tot_length
    }
}

pub fn find_shortest_dist<T: Graphable>(
    graph: &T,
    start: &T::Coordinates,
    end: &T::Coordinates,
) -> Option<i64> {
    let mut heads = VecDeque::new();
    let mut seen = HashMap::new();

    let start_node = graph.node_at(start).unwrap();
    heads.push_front((0, start_node.clone()));
    seen.insert(start_node.clone(), 0);

    loop {
        let o_head = heads.pop_front();
        let head;
        match o_head {
            None => {
                return None;
            }
            Some(h) => head = h,
        }
        let path_length = head.0 + 1;
        for (neighbour, _weight) in graph.neighbours(&head.1) {
            let min_path_to_neighbour_length = seen.get(&neighbour).unwrap_or(&std::i64::MAX);
            if path_length < *min_path_to_neighbour_length {
                heads.push_back((path_length, neighbour.clone()));
                seen.insert(neighbour.clone(), path_length);

                if graph.coords_for(&neighbour) == *end {
                    return Some(head.0 + 1);
                }
            }
        }
    }
}

pub fn find_shortest_path<T: Graphable + Clone + Debug>(
    graph: &T,
    start: &T::Coordinates,
    end: &T::Coordinates,
) -> Option<Path<T>> {
    let mut paths: VecDeque<Path<T>> = VecDeque::new();
    let mut min_length_to: HashMap<Rc<T::Node>, i64> = HashMap::new();
    let mut soluces: VecDeque<Path<T>> = VecDeque::new();

    // Start with a single path of length 0, at the starting node
    let start_node = graph.node_at(start).unwrap();
    let mut starting_nodes = HashMap::new();
    starting_nodes.insert(start_node.clone(), 0);
    paths.push_front(Path {
        head: start_node.clone(),
        node_path: vec![start_node.clone()],
        nodes: starting_nodes,
        tot_length: 0,
    });
    min_length_to.insert(start_node.clone(), 0);

    loop {
        // Takes the first path. It assumes that paths is sorted by total_length. We try to grow only
        // the shortest path
        let opt_to_grow = paths.pop_front();
        let to_grow: Path<T>;

        match opt_to_grow {
            None => break,
            Some(p) => to_grow = p,
        }
        debug::debug(format!("\nGrowing {:?}", to_grow));

        // The first path in soluce (if any) is the shortest solution found so far.
        // If our next path to grow is longer, then we have already found the best soluce
        let best_soluce = soluces.get(0);
        match best_soluce {
            None => (),
            Some(soluce) => {
                if soluce.tot_length < to_grow.tot_length {
                    debug::debug(format!(
                        "Trying to grow {:?} that is longer than soluce {:?}. Stopping",
                        to_grow, soluce
                    ));
                    break;
                }
                ()
            }
        }

        // Checking if this path is still interesting
        let head = to_grow.head.clone();
        let best_path_to_head = min_length_to.get(&head).unwrap_or(&std::i64::MAX);
        if to_grow.tot_length > *best_path_to_head {
            debug::debug(format!("Path {:?} is now obsolete, skipping", to_grow));
            break;
        }

        // Try to add each of its head's neighbours
        for (neighbour, distance) in graph.neighbours(&to_grow.head) {
            let mut new_path = to_grow.clone();
            let res = new_path.grow(neighbour.clone(), distance);
            debug::debug(format!(
                "Trying head {:?} for a new path {:?} at length {}",
                neighbour, new_path.node_path, new_path.tot_length
            ));
            if res.is_err() {
                debug::debug(format!("Path is looping, discarding"));
                continue;
            }

            // Check if we have already seen this head
            let seen_head = min_length_to.get(&neighbour);
            match seen_head {
                None => {
                    // replace the shortest path
                    min_length_to.insert(neighbour.clone(), new_path.tot_length);
                    // if we have reached the end, do not try to grow this path anymore
                    if graph.coords_for(&neighbour) == *end {
                        let insert_at =
                            soluces.partition_point(|path| path.tot_length < new_path.tot_length);
                        soluces.insert(insert_at, new_path);
                        debug::debug(format!("Kept"));
                        debug::debug(format!(
                            "Inserting in soluces at {} - {:?}",
                            insert_at, &soluces
                        ));
                    } else {
                        // adds the new path in the right place (sorted)
                        let insert_at =
                            paths.partition_point(|path| path.tot_length < new_path.tot_length);
                        paths.insert(insert_at, new_path);
                        debug::debug(format!("Kept"));
                        debug::debug(format!(
                            "Inserting in paths at {} - {:?}",
                            insert_at, &paths
                        ));
                    }
                }
                Some(seen_length) => {
                    // Check if the new path is shorter
                    // If it is, truncate all the other paths that go through this head in a longer
                    // way
                    // If it is not, drop the path
                    if *seen_length <= new_path.tot_length {
                        debug::debug(format!("Discarded due to better path already seen"));
                        continue;
                    } else {
                        debug::debug(format!(
                            "New shortest path to {:?}, of length {}",
                            &neighbour, new_path.tot_length
                        ));
                        // replace the shortest path
                        min_length_to.insert(neighbour.clone(), new_path.tot_length);
                        // if we have reached the end, do not try to grow this path anymore
                        if graph.coords_for(&neighbour) == *end {
                            let insert_at = soluces
                                .partition_point(|path| path.tot_length < new_path.tot_length);
                            soluces.insert(insert_at, new_path);
                            debug::debug(format!("Kept"));
                            debug::debug(format!(
                                "Inserting in soluces at {} - {:?}",
                                insert_at, &soluces
                            ));
                        } else {
                            // adds the new path in the right place (sorted)
                            let insert_at =
                                paths.partition_point(|path| path.tot_length < new_path.tot_length);
                            paths.insert(insert_at, new_path);
                            debug::debug(format!("Kept"));
                            debug::debug(format!(
                                "Inserting in paths at {} - {:?}",
                                insert_at, &paths
                            ));
                        }
                    }
                }
            }
        }
        debug::debug(format!("{:?}", &paths));
    }

    soluces.get(0).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct Coordinates {
        x: i64,
        y: i64,
    }

    #[derive(Hash, PartialEq, Eq, Clone)]
    struct Node {
        coord: Coordinates,
        passable: bool,
    }

    impl Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("({}, {})", self.coord.x, self.coord.y))
        }
    }

    #[derive(Debug, Clone)]
    struct Graph {
        nodes: HashMap<Coordinates, Rc<Node>>,
    }

    impl Graphable for Graph {
        type Node = Node;
        type Coordinates = Coordinates;

        fn coords_for(&self, node: &Self::Node) -> Self::Coordinates {
            node.coord.clone()
        }

        fn node_at(&self, coords: &Self::Coordinates) -> Option<Rc<Self::Node>> {
            match self.nodes.get(coords) {
                Some(node) => Some(node.clone()),
                None => None,
            }
        }

        fn neighbours(&self, node: &Self::Node) -> Vec<(Rc<Self::Node>, i64)> {
            let mut res = Vec::new();
            for i in -1i64..=1 {
                for j in -1i64..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    // cannot move diagonally
                    if i != 0 && j != 0 {
                        continue;
                    }
                    let n_coords = Coordinates {
                        x: node.coord.x + i,
                        y: node.coord.y + j,
                    };
                    let neighbour = self.node_at(&n_coords);
                    match neighbour {
                        Some(n) => {
                            if n.passable {
                                res.push((n.clone(), 1));
                            }
                        }
                        None => (),
                    }
                }
            }

            res
        }
    }

    // S . . .
    // # . # #
    // . . . #
    // . # . #
    // . . . E

    #[test]
    fn test_equi_weight() {
        let mut graph = Graph {
            nodes: HashMap::new(),
        };

        for x in 0..4 {
            for y in 0..5 {
                let mut passable = true;
                if HashSet::<Coordinates>::from_iter(vec![
                    Coordinates { x: 0, y: 1 },
                    Coordinates { x: 2, y: 1 },
                    Coordinates { x: 3, y: 1 },
                    Coordinates { x: 3, y: 2 },
                    Coordinates { x: 1, y: 3 },
                    Coordinates { x: 3, y: 3 },
                ])
                .get(&Coordinates { x, y })
                .is_some()
                {
                    passable = false;
                }
                graph.nodes.insert(
                    Coordinates { x, y },
                    Rc::new(Node {
                        coord: Coordinates { x, y },
                        passable,
                    }),
                );
            }
        }

        let res = find_shortest_dist(
            &graph,
            &Coordinates { x: 0, y: 0 },
            &Coordinates { x: 3, y: 4 },
        );
        assert!(res.is_some());
        assert!(res.unwrap() == 7);

        let res = find_shortest_path(
            &graph,
            &Coordinates { x: 0, y: 0 },
            &Coordinates { x: 3, y: 4 },
        );
        assert!(res.is_some());
        let path = res.unwrap();
        assert!(*&path.nodes.len() == 8);
        assert!(*&path.tot_length == 7);
    }
}
