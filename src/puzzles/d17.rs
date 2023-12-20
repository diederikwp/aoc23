use std::{cmp::Reverse, collections::BinaryHeap, error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Eq, PartialEq)]
pub struct Map(Array2<u8>);

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let linear_grid = Array::from_iter(s.bytes().filter(|&b| b != b'\n').map(|b| b - b'0'));
        let height = linear_grid.len() / width;

        Ok(Map(linear_grid.into_shape((height, width))?))
    }
}

impl Map {
    pub fn cheapest_path_cost_normal(&self) -> Option<u32> {
        self.cheapest_path_cost::<Crucible>()
    }

    pub fn cheapest_path_cost_ultra(&self) -> Option<u32> {
        self.cheapest_path_cost::<UltraCrucible>()
    }

    /// Find the cost of the shortest path using the A* algorithm
    fn cheapest_path_cost<T: Node>(&self) -> Option<u32> {
        // visited contains nodes fully expanded
        let mut visited = FxHashSet::default();
        // The frontier contains nodes discovered but not fully expanded yet, as
        // tuples of (heuristic_cost_start_to_target_through_node, cost_node,
        // node). The first element of the tuple is used for ordering in the
        // heap (Reverse is used to make a min-heap).
        let mut frontier = BinaryHeap::new();
        // best_cost contains the lowest cost from start to node, for every
        // discovered node.
        let mut best_cost = FxHashMap::default();

        // start direction South disallows turning back North, but that is
        // ok because that would take us of the map.
        let start_node = T::new((0, 0), Direction::South);
        let start_heuristic = Self::manhattan_dist((0, 0), self.target());

        frontier.push(Reverse((start_heuristic, 0, start_node.clone())));
        best_cost.insert(start_node, 0);

        while let Some(Reverse((_, cost, node))) = frontier.pop() {
            if node.pos() == self.target() && node.can_stop() {
                return Some(cost);
            }

            for maybe_neighbour in node.get_all_neighbours(self) {
                let Some(neighbour) = maybe_neighbour else {
                    continue;
                };

                if visited.contains(&neighbour) {
                    continue; // We already visited this node
                }

                let neighbour_cost = cost + u32::from(self.0[neighbour.pos()]);
                if best_cost
                    .get(&neighbour)
                    .is_some_and(|&c| c <= neighbour_cost)
                {
                    continue; // This node is already on the frontier with an equal or better path
                }
                best_cost.insert(neighbour.clone(), neighbour_cost);

                let neighbour_heuristic_total = neighbour_cost + neighbour.heuristic(self);
                frontier.push(Reverse((
                    neighbour_heuristic_total,
                    neighbour_cost,
                    neighbour,
                )));
            }
            visited.insert(node);
        }

        // Target position is not reachable from start
        None
    }

    fn target(&self) -> (usize, usize) {
        (self.0.shape()[0] - 1, self.0.shape()[1] - 1)
    }

    fn manhattan_dist(from: (usize, usize), to: (usize, usize)) -> u32 {
        u32::try_from(from.0.abs_diff(to.0) + from.1.abs_diff(to.1)).unwrap()
    }
}

trait Node: Clone + std::hash::Hash + Ord + Sized {
    fn new(start_pos: (usize, usize), start_direction: Direction) -> Self;
    fn pos(&self) -> (usize, usize);
    fn get_all_neighbours(&self, map: &Map) -> [Option<Self>; 4];
    fn can_stop(&self) -> bool;
    fn heuristic(&self, map: &Map) -> u32;
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Crucible {
    pos: (usize, usize),
    direction: Direction,
    remaining_steps: u8, // How many more steps are allowed in direction
}

impl Node for Crucible {
    fn new(start_pos: (usize, usize), start_direction: Direction) -> Self {
        Crucible {
            pos: start_pos,
            direction: start_direction,
            remaining_steps: 3,
        }
    }

    fn pos(&self) -> (usize, usize) {
        self.pos
    }

    fn get_all_neighbours(&self, map: &Map) -> [Option<Crucible>; 4] {
        [
            self.get_neighbour(Direction::North, map),
            self.get_neighbour(Direction::East, map),
            self.get_neighbour(Direction::South, map),
            self.get_neighbour(Direction::West, map),
        ]
    }

    fn can_stop(&self) -> bool {
        true
    }

    fn heuristic(&self, map: &Map) -> u32 {
        Map::manhattan_dist(self.pos, map.target())
    }
}

impl Crucible {
    fn get_neighbour(&self, direction: Direction, map: &Map) -> Option<Crucible> {
        let (height, width) = (map.0.shape()[0], map.0.shape()[1]);

        let pos = match direction {
            Direction::North => {
                if self.pos.0 > 0
                    && self.direction != Direction::South
                    && (self.direction != Direction::North || self.remaining_steps > 0)
                {
                    Some((self.pos.0 - 1, self.pos.1))
                } else {
                    None
                }
            }
            Direction::East => {
                if self.pos.1 < width - 1
                    && self.direction != Direction::West
                    && (self.direction != Direction::East || self.remaining_steps > 0)
                {
                    Some((self.pos.0, self.pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::South => {
                if self.pos.0 < height - 1
                    && self.direction != Direction::North
                    && (self.direction != Direction::South || self.remaining_steps > 0)
                {
                    Some((self.pos.0 + 1, self.pos.1))
                } else {
                    None
                }
            }
            Direction::West => {
                if self.pos.1 > 0
                    && self.direction != Direction::East
                    && (self.direction != Direction::West || self.remaining_steps > 0)
                {
                    Some((self.pos.0, self.pos.1 - 1))
                } else {
                    None
                }
            }
        }?;

        let remaining_steps = if self.direction == direction {
            self.remaining_steps - 1
        } else {
            2
        };

        Some(Crucible {
            pos,
            direction,
            remaining_steps,
        })
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct UltraCrucible {
    pos: (usize, usize),
    direction: Direction,
    consecutive_steps: u8, // how many consecutive steps in direction it already performed
}

impl Node for UltraCrucible {
    fn new(start_pos: (usize, usize), start_direction: Direction) -> Self {
        UltraCrucible {
            pos: start_pos,
            direction: start_direction,
            consecutive_steps: 0,
        }
    }

    fn pos(&self) -> (usize, usize) {
        self.pos
    }

    fn get_all_neighbours(&self, map: &Map) -> [Option<Self>; 4] {
        [
            self.get_neighbour(Direction::North, map),
            self.get_neighbour(Direction::East, map),
            self.get_neighbour(Direction::South, map),
            self.get_neighbour(Direction::West, map),
        ]
    }

    fn can_stop(&self) -> bool {
        self.consecutive_steps >= 4
    }

    fn heuristic(&self, map: &Map) -> u32 {
        Map::manhattan_dist(self.pos, map.target())
    }
}

impl UltraCrucible {
    fn get_neighbour(&self, direction: Direction, map: &Map) -> Option<UltraCrucible> {
        let (height, width) = (map.0.shape()[0], map.0.shape()[1]);

        let pos = match direction {
            Direction::North => {
                if self.pos.0 > 0
                    && self.direction != Direction::South
                    // Starting node is a special case; it can turn even though
                    // it has not made 4 consecutive steps yet.
                    && (self.consecutive_steps == 0
                        || (self.direction != Direction::North && self.consecutive_steps >= 4)
                        || (self.direction == Direction::North && self.consecutive_steps < 10))
                {
                    Some((self.pos.0 - 1, self.pos.1))
                } else {
                    None
                }
            }
            Direction::East => {
                if self.pos.1 < width - 1
                    && self.direction != Direction::West
                    && (self.consecutive_steps == 0
                        || (self.direction != Direction::East && self.consecutive_steps >= 4)
                        || (self.direction == Direction::East && self.consecutive_steps < 10))
                {
                    Some((self.pos.0, self.pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::South => {
                if self.pos.0 < height - 1
                    && self.direction != Direction::North
                    && (self.consecutive_steps == 0
                        || (self.direction != Direction::South && self.consecutive_steps >= 4)
                        || (self.direction == Direction::South && self.consecutive_steps < 10))
                {
                    Some((self.pos.0 + 1, self.pos.1))
                } else {
                    None
                }
            }
            Direction::West => {
                if self.pos.1 > 0
                    && self.direction != Direction::East
                    && (self.consecutive_steps == 0
                        || (self.direction != Direction::West && self.consecutive_steps >= 4)
                        || (self.direction == Direction::West && self.consecutive_steps < 10))
                {
                    Some((self.pos.0, self.pos.1 - 1))
                } else {
                    None
                }
            }
        }?;

        let consecutive_steps = if self.direction == direction {
            self.consecutive_steps + 1
        } else {
            1
        };

        Some(UltraCrucible {
            pos,
            direction,
            consecutive_steps,
        })
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}
