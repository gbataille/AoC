use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;
use std::vec::Vec;

pub trait Graphable {
    type Node: Hash + Eq + Debug;
    type Coordinates: Eq + Debug;

    fn coords_for(&self, node: &Self::Node) -> Self::Coordinates;
    fn node_at(&self, coords: &Self::Coordinates) -> Option<Rc<Self::Node>>;
    fn neighbours(&self, node: &Self::Node) -> Vec<(Rc<Self::Node>, i64)>;
}

pub fn find_path<T: Graphable>(
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    struct Coordinates {
        x: i64,
        y: i64,
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Node {
        coord: Coordinates,
        passable: bool,
    }

    #[derive(Debug)]
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

        let res = find_path(
            &graph,
            &Coordinates { x: 0, y: 0 },
            &Coordinates { x: 3, y: 4 },
        );
        assert!(res.is_some());
        assert!(res.unwrap() == 7);
    }
}
