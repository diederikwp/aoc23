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
        // ok because that would take us off the map.
        let start_node = T::new((0, 0), Direction::South);
        let start_heuristic = Self::manhattan_dist((0, 0), self.target());

        frontier.push(Reverse((start_heuristic, 0, start_node.clone())));
        best_cost.insert(start_node, 0);

        while let Some(Reverse((_, cost, node))) = frontier.pop() {
            if node.pos() == self.target() && node.can_stop() {
                return Some(cost);
            }

            for neighbour in node.get_all_neighbours(self).into_iter().flatten() {
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

    fn get_neighbour_pos(
        &self,
        pos: (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        let ipos = (
            isize::try_from(pos.0).unwrap(),
            isize::try_from(pos.1).unwrap(),
        );
        let iheight = isize::try_from(self.0.shape()[0]).unwrap();
        let iwidth = isize::try_from(self.0.shape()[1]).unwrap();

        let neighbour_pos = (ipos.0 + direction.dy(), ipos.1 + direction.dx());
        if neighbour_pos.0 < 0
            || neighbour_pos.1 < 0
            || neighbour_pos.0 >= iheight
            || neighbour_pos.1 >= iwidth
        {
            return None;
        }

        Some((
            usize::try_from(neighbour_pos.0).unwrap(),
            usize::try_from(neighbour_pos.1).unwrap(),
        ))
    }
}

trait Node: Clone + std::hash::Hash + Ord + Sized {
    fn new(start_pos: (usize, usize), start_direction: Direction) -> Self;
    fn pos(&self) -> (usize, usize);
    fn direction(&self) -> Direction;
    fn can_stop(&self) -> bool;
    fn make_step(&self, map: &Map, direction: Direction) -> Option<Self>;
    fn heuristic(&self, map: &Map) -> u32;

    fn get_all_neighbours(&self, map: &Map) -> [Option<Self>; 4] {
        [
            self.make_step(map, Direction::North),
            self.make_step(map, Direction::East),
            self.make_step(map, Direction::South),
            self.make_step(map, Direction::West),
        ]
    }
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

    fn direction(&self) -> Direction {
        self.direction
    }

    fn can_stop(&self) -> bool {
        true
    }

    fn make_step(&self, map: &Map, direction: Direction) -> Option<Self> {
        // neighbour is on map
        let neighbour_pos = map.get_neighbour_pos(self.pos(), direction)?;

        // turn is allowed (not 180)
        if self.direction == direction.opposite() {
            return None;
        }

        // don't exceed remaining steps
        if self.direction == direction && self.remaining_steps == 0 {
            return None;
        }

        // update remaining steps
        let remaining_steps = if self.direction == direction {
            self.remaining_steps - 1
        } else {
            2
        };

        Some(Crucible {
            pos: neighbour_pos,
            direction,
            remaining_steps,
        })
    }

    fn heuristic(&self, map: &Map) -> u32 {
        Map::manhattan_dist(self.pos, map.target())
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

    fn direction(&self) -> Direction {
        self.direction
    }

    fn can_stop(&self) -> bool {
        self.consecutive_steps >= 4
    }

    fn make_step(&self, map: &Map, direction: Direction) -> Option<Self> {
        // neighbour is on map
        let neighbour_pos = map.get_neighbour_pos(self.pos(), direction)?;

        // turn is allowed (not 180)
        if self.direction == direction.opposite() {
            return None;
        }

        // satisfy consecutive steps constraint. Special case: starting node can
        // always turn even if it has not made 4 steps yet.
        if self.direction == direction
            && self.consecutive_steps >= 10
            && self.consecutive_steps != 0
        {
            return None;
        }
        if self.direction != direction && self.consecutive_steps < 4 && self.consecutive_steps != 0
        {
            return None;
        }

        // update consecutive steps
        let consecutive_steps = if self.direction == direction {
            self.consecutive_steps + 1
        } else {
            1
        };

        Some(UltraCrucible {
            pos: neighbour_pos,
            direction,
            consecutive_steps,
        })
    }

    fn heuristic(&self, map: &Map) -> u32 {
        Map::manhattan_dist(self.pos, map.target())
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn dx(&self) -> isize {
        match self {
            Direction::North | Direction::South => 0,
            Direction::East => 1,
            Direction::West => -1,
        }
    }

    fn dy(&self) -> isize {
        match self {
            Direction::East | Direction::West => 0,
            Direction::North => -1,
            Direction::South => 1,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}
