use num::traits::Zero;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;
use std::vec::Vec;

pub trait Graphable {
    type Node: Hash + Eq + Debug + Clone;
    type Coordinates: Eq + Debug + Clone;
    type PathWeight: Hash + Eq + Debug + Clone + Add + Zero + Ord;

    fn coords_for(&self, node: &Self::Node) -> Self::Coordinates;
    fn node_at(&self, coords: &Self::Coordinates) -> Option<Rc<Self::Node>>;
    fn neighbours(&self, node: &Self::Node) -> Vec<(Rc<Self::Node>, Self::PathWeight)>;
}

#[derive(Debug)]
pub struct PathFinder<'a, T: Graphable> {
    start_node: Rc<T::Node>,
    end_node: Rc<T::Node>,
    graph: &'a T,

    ordered_heads: VecDeque<Rc<PathElem<T>>>,
    best_heads_index: HashMap<Rc<T::Node>, T::PathWeight>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct PathElem<T: Graphable> {
    node: Rc<T::Node>,
    length_to_node: T::PathWeight,
    previous_node: Option<Rc<PathElem<T>>>,
}

impl<'a, T: Graphable> PathFinder<'a, T> {
    pub fn new(
        start_node_coord: &T::Coordinates,
        end_node_coord: &T::Coordinates,
        graph: &'a T,
    ) -> Result<Self, ()> {
        let start_node = graph.node_at(start_node_coord).ok_or(())?;
        let end_node = graph.node_at(end_node_coord).ok_or(())?;

        let start_path_elem = Rc::new(PathElem {
            node: start_node.clone(),
            length_to_node: T::PathWeight::zero(),
            previous_node: None,
        });

        let mut best_heads_index = HashMap::new();
        best_heads_index.insert(start_node.clone(), T::PathWeight::zero());

        let mut ordered_heads = VecDeque::new();
        ordered_heads.push_back(start_path_elem.clone());

        Ok(PathFinder {
            start_node: start_node.clone(),
            end_node: end_node.clone(),
            graph,
            ordered_heads,
            best_heads_index,
        })
    }

    pub fn reset_to(
        &mut self,
        start_node_coord: &T::Coordinates,
        end_node_coord: &T::Coordinates,
    ) -> Result<(), ()> {
        let start_node = self.graph.node_at(start_node_coord).ok_or(())?;
        let end_node = self.graph.node_at(end_node_coord).ok_or(())?;
        self.start_node = start_node.clone();
        self.end_node = end_node.clone();
        Ok(())
    }

    pub fn find_shortest_dist(&mut self) -> Option<T::PathWeight> {
        let res = self.solve()?;
        Some(res.length_to_node.clone())
    }

    pub fn find_shortest_path(&mut self) -> Option<(Vec<Rc<T::Node>>, T::PathWeight)> {
        let head = self.solve()?;
        let length = head.length_to_node.clone();

        let mut path: Vec<Rc<T::Node>> = Vec::new();

        let mut cur_elem = head;
        loop {
            path.push(cur_elem.node.clone());
            match &cur_elem.previous_node {
                None => break,
                Some(elem) => cur_elem = elem.clone(),
            }
        }
        path.reverse();
        Some((path, length))
    }

    fn solve(&mut self) -> Option<Rc<PathElem<T>>> {
        loop {
            let head = self.ordered_heads.pop_front()?;

            // if the next (i.e. next shortest path) is the end, that means that we are done
            if self.graph.coords_for(head.node.as_ref())
                == self.graph.coords_for(self.end_node.as_ref())
            {
                return Some(head.clone());
            }

            for (neighbour, weight) in self.graph.neighbours(head.node.as_ref()) {
                let new_path_length = head.length_to_node.clone() + weight;

                let min_path_to_neighbour_length = self.best_heads_index.get(&neighbour);

                match min_path_to_neighbour_length {
                    None => (),
                    Some(length) => {
                        if new_path_length >= *length {
                            // we already have a better solution
                            continue;
                        }
                    }
                }

                self.best_heads_index
                    .insert(neighbour.clone(), new_path_length.clone());
                let insert_at = self
                    .ordered_heads
                    .partition_point(|elem| elem.length_to_node < new_path_length);
                self.ordered_heads.insert(
                    insert_at,
                    Rc::new(PathElem {
                        node: neighbour.clone(),
                        length_to_node: new_path_length,
                        previous_node: Some(head.clone()),
                    }),
                );
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

        let mut pf = PathFinder::new(
            &Coordinates { x: 0, y: 0 },
            &Coordinates { x: 3, y: 4 },
            &graph,
        )
        .unwrap();
        let res = pf.find_shortest_dist();
        assert!(res.is_some());
        assert!(res.unwrap() == 7);
    }
}
