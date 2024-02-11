use std::{cmp::Reverse, collections::BinaryHeap, error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

#[derive(Eq, PartialEq)]
pub struct Map {
    grid: Array2<u8>,

    /// Shortest path from position to exit, taking into account heat loss but
    /// no "consecutive steps" constraints.
    heur_cost_to_target: Array2<u32>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());
        let linear_grid = Array::from_iter(s.bytes().filter(|&b| b != b'\n').map(|b| b - b'0'));
        let height = linear_grid.len() / width;
        let grid = linear_grid.into_shape((height, width))?;
        let heur_cost_to_target = Self::find_lowest_cost_to_target(&grid);

        Ok(Map {
            grid,
            heur_cost_to_target,
        })
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
        let mut visited = HashSet::default();
        // The frontier contains nodes discovered but not fully expanded yet, as
        // tuples of (heuristic_cost_start_to_target_through_node, cost_node,
        // node). The first element of the tuple is used for ordering in the
        // heap (Reverse is used to make a min-heap).
        let mut frontier = BinaryHeap::new();
        // best_cost contains the lowest cost from start to node, for every
        // discovered node.
        let mut best_cost = HashMap::default();

        // start direction South disallows turning back North, but that is
        // ok because that would take us off the map.
        let start_node = T::new((0, 0), Direction::South);
        let start_heuristic = self.heur_cost_to_target[(0, 0)];

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

                let neighbour_cost = cost + u32::from(self.grid[neighbour.pos()]);
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
        (self.grid.shape()[0] - 1, self.grid.shape()[1] - 1)
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
        let iheight = isize::try_from(self.grid.shape()[0]).unwrap();
        let iwidth = isize::try_from(self.grid.shape()[1]).unwrap();

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

    /// Find the shortest path from any position to the target position using
    /// Dijkstra's algorithm. This takes into account heat loss but no
    /// "consecutive steps" constraints.
    fn find_lowest_cost_to_target(grid: &Array2<u8>) -> Array2<u32> {
        let shape = [grid.shape()[0], grid.shape()[1]];
        let target = (shape[0] - 1, shape[1] - 1);

        let mut lowest_cost = Array2::from_elem(shape, u32::MAX);
        let mut visited = Array2::from_elem(shape, false);
        let mut frontier = BinaryHeap::new(); // contains (pos, cost) tuples
        lowest_cost[target] = 0;
        frontier.push(Reverse((0, target)));

        while let Some(Reverse((cost, pos))) = frontier.pop() {
            if visited[pos] {
                continue;
            }

            for (dy, dx) in [(1, 0), (0, -1), (-1, 0), (0, 1)] {
                let ineighbour = (pos.0 as isize + dy, pos.1 as isize + dx);
                if ineighbour.0 < 0 || ineighbour.1 < 0 {
                    continue;
                }
                let neighbour = (ineighbour.0 as usize, ineighbour.1 as usize);
                if neighbour.0 >= shape[0] || neighbour.1 >= shape[1] {
                    continue;
                }

                let neighbour_cost = cost + u32::from(grid[pos]);
                if neighbour_cost < lowest_cost[neighbour] {
                    lowest_cost[neighbour] = neighbour_cost;
                }

                frontier.push(Reverse((lowest_cost[neighbour], neighbour)));
            }

            visited[pos] = true;
        }

        lowest_cost
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
        map.heur_cost_to_target[self.pos]
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
        map.heur_cost_to_target[self.pos]
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
