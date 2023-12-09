use std::{error::Error, str::FromStr};

use num_integer::lcm;
use rustc_hash::{FxHashMap, FxHashSet};

pub struct Network {
    instructions: Vec<Direction>,
    edges: FxHashMap<Node, (Node, Node)>,
}

impl Network {
    pub fn n_steps_from_to_single(&self, from: &str, to: &str) -> u32 {
        let from_node: Node = from.parse().unwrap();
        let to_node: Node = to.parse().unwrap();

        let mut node = from_node;
        let mut steps = 0;
        let mut directions = self.instructions.iter().cycle();
        while node != to_node {
            let direction = directions.next().unwrap();
            node = match direction {
                Direction::Left => self.edges[&node].0.clone(),
                Direction::Right => self.edges[&node].1.clone(),
            };
            steps += 1;
        }

        steps
    }

    pub fn n_steps_all_a_to_all_z(&self) -> u64 {
        let a_nodes: Vec<Node> = self
            .edges
            .keys()
            .filter(|n| n.0[2] == 'A')
            .cloned()
            .collect();
        let z_nodes: FxHashSet<Node> = self
            .edges
            .keys()
            .filter(|n| n.0[2] == 'Z')
            .cloned()
            .collect();

        a_nodes
            .iter()
            .map(|n| self.n_steps_from_to_multiple(n, &z_nodes))
            .map(u64::from)
            .reduce(lcm)
            .unwrap()
    }

    fn n_steps_from_to_multiple(&self, from: &Node, to: &FxHashSet<Node>) -> u32 {
        let mut node = from;
        let mut directions = self.instructions.iter().cycle();

        let mut steps = 0;
        while !to.contains(node) {
            let direction = directions.next().unwrap();
            node = match direction {
                Direction::Left => &self.edges[node].0,
                Direction::Right => &self.edges[node].1,
            };
            steps += 1;
        }

        steps
    }
}

impl FromStr for Network {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let instructions = lines
            .next()
            .ok_or("No first line in input")?
            .chars()
            .map(Direction::new)
            .collect::<Option<Vec<_>>>()
            .ok_or("Invalid direction in input")?;

        let mut edges = FxHashMap::default();
        for line in lines.skip(1) {
            let (from_str, to_str) = line.split_once(" = ").ok_or("Invalid syntax")?;

            // Parse 1 "from" node
            let from_node: Node = from_str.parse()?;

            // Remove brackets and parse 2 "to" nodes
            let to_str = to_str.get(1..(to_str.len() - 1)).ok_or("Invalid syntax")?;
            let (to_str_l, to_str_r) = to_str.split_once(", ").ok_or("Invalid syntax")?;
            let to_node_l = to_str_l.parse()?;
            let to_node_r = to_str_r.parse()?;

            // Add to edges
            edges.insert(from_node, (to_node_l, to_node_r));
        }

        Ok(Network {
            instructions,
            edges,
        })
    }
}

enum Direction {
    Right,
    Left,
}

impl Direction {
    fn new(c: char) -> Option<Self> {
        match c {
            'R' => Some(Self::Right),
            'L' => Some(Self::Left),
            _ => None,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Node([char; 3]);

impl FromStr for Node {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            Err("Node should have 3 characters".into())
        } else {
            let chars = [0, 1, 2].map(|i| s.chars().nth(i).unwrap());
            Ok(Node(chars))
        }
    }
}
