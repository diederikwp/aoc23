use std::collections::VecDeque;
use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

type Pos = (usize, usize);

pub struct Map {
    grid: Grid,
    edges_out: HashMap<Pos, Vec<(Pos, u32)>>, // key: from_position, value: pairs of (to_position, distance)
    edges_in: HashMap<Pos, Vec<(Pos, u32)>>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid = s.parse()?;
        Ok(Self::from_grid(grid))
    }
}

impl Map {
    pub fn longest_path_len(&self) -> u32 {
        // Assumption: graph is acyclic (a DAG)
        let topsort = self.topological_sort();
        let mut max_total_dist: HashMap<Pos, u32> = HashMap::default();
        max_total_dist.insert(self.grid.entrance(), 0);

        for vx in topsort {
            let Some(parents) = self.edges_in.get(&vx) else {
                continue;
            };

            for (parent, parent_dist) in parents {
                let total_dist_through_parent = parent_dist + max_total_dist[&parent];
                max_total_dist
                    .entry(vx)
                    .and_modify(|d| *d = u32::max(*d, total_dist_through_parent))
                    .or_insert(total_dist_through_parent);
            }
        }

        max_total_dist[&self.grid.exit()]
    }

    fn from_grid(grid: Grid) -> Self {
        let vertices = grid.find_vertices();
        let edges_out = grid.find_edges_out(&vertices);
        let edges_in = Grid::find_edges_in_from_edges_out(&edges_out);
        Map {
            grid,
            edges_out,
            edges_in,
        }
    }

    fn topological_sort(&self) -> Vec<Pos> {
        // using DFS. Assuming acyclic graph.
        let mut topsort = VecDeque::new();
        let mut visited = HashSet::default();
        let mut stack = vec![(self.grid.entrance(), false)];

        while let Some((pos, finished)) = stack.pop() {
            if finished {
                topsort.push_front(pos);
                visited.insert(pos);
                continue;
            }

            if visited.contains(&pos) {
                continue;
            }

            stack.push((pos, true));

            let Some(children) = self.edges_out.get(&pos) else {
                continue;
            };
            for (neighbour, _dist) in children {
                if !visited.contains(neighbour) {
                    stack.push((*neighbour, false));
                }
            }
        }

        topsort.into()
    }
}

struct Grid(Array2<u8>);

impl FromStr for Grid {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let linear_grid = s
            .bytes()
            .filter(|&b| b != b'\n')
            .map(|b| match b {
                b'#' | b'.' | b'^' | b'>' | b'v' | b'<' => Ok(b),
                _ => Err("Invalid character"),
            })
            .collect::<Result<Vec<u8>, _>>()?;

        let width = s.find('\n').unwrap_or(s.len());
        let height = linear_grid.len() / width;
        let grid = Array::from_vec(linear_grid).into_shape((height, width))?;

        Ok(Grid(grid))
    }
}

impl Grid {
    fn height(&self) -> usize {
        self.0.shape()[0]
    }

    fn width(&self) -> usize {
        self.0.shape()[1]
    }

    fn entrance(&self) -> Pos {
        (0, 1)
    }

    fn exit(&self) -> Pos {
        (self.height() - 1, self.width() - 2)
    }

    fn neighbours(&self, pos: &Pos) -> [Option<Pos>; 4] {
        let (y, x) = pos;

        [(0, 1), (1, 0), (0, -1), (-1, 0)].map(|(dy, dx)| {
            let neighbour_pos = (y.checked_add_signed(dy)?, x.checked_add_signed(dx)?);
            if neighbour_pos.0 >= self.height() || neighbour_pos.1 >= self.width() {
                return None;
            }

            if [b'.', b'^', b'>', b'v', b'<'].contains(&self.0[neighbour_pos]) {
                Some(neighbour_pos)
            } else {
                None
            }
        })
    }

    fn find_vertices(&self) -> HashSet<Pos> {
        let height = self.0.shape()[0];
        let width = self.0.shape()[1];

        let mut vertices = HashSet::default();
        vertices.insert(self.entrance());
        vertices.insert(self.exit());

        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                if self.0[(y, x)] == b'#' {
                    continue;
                }

                let n_neighbours = self.neighbours(&(y, x)).iter().flatten().count();
                if n_neighbours > 2 {
                    vertices.insert((y, x));
                }
            }
        }

        vertices
    }

    fn find_edges_out(&self, vertices: &HashSet<Pos>) -> HashMap<Pos, Vec<(Pos, u32)>> {
        let mut edges: HashMap<Pos, Vec<(Pos, u32)>> = HashMap::default();

        for vx in vertices {
            for neighbour_pos in self.neighbours(vx).into_iter().flatten() {
                let direction = (
                    isize::try_from(neighbour_pos.0).unwrap() - isize::try_from(vx.0).unwrap(),
                    isize::try_from(neighbour_pos.1).unwrap() - isize::try_from(vx.1).unwrap(),
                );

                if let Some((target_vx, steps)) = self.walk_to_next_vertex(vx, direction, vertices)
                {
                    edges.entry(*vx).or_default().push((target_vx, steps));
                }
            }
        }

        edges
    }

    fn find_edges_in_from_edges_out(
        edges_out: &HashMap<Pos, Vec<(Pos, u32)>>,
    ) -> HashMap<Pos, Vec<(Pos, u32)>> {
        let mut edges_in: HashMap<Pos, Vec<(Pos, u32)>> = HashMap::default();
        for (&node_from, outgoing_edges) in edges_out {
            for &(node_to, distance) in outgoing_edges {
                edges_in
                    .entry(node_to)
                    .or_default()
                    .push((node_from, distance));
            }
        }

        edges_in
    }

    fn walk_to_next_vertex(
        &self,
        start_pos: &Pos,
        start_direction: (isize, isize),
        vertices: &HashSet<Pos>,
    ) -> Option<(Pos, u32)> {
        let mut direction = start_direction;
        let mut pos = *start_pos;
        let mut n_steps = 0;

        loop {
            let next_pos = (
                pos.0.checked_add_signed(direction.0).unwrap(),
                pos.1.checked_add_signed(direction.1).unwrap(),
            );
            let mut do_move = true;

            match self.0[next_pos] {
                b'.' => (),
                b'^' => {
                    if direction.0 != -1 {
                        return None;
                    }
                }
                b'>' => {
                    if direction.1 != 1 {
                        return None;
                    }
                }
                b'v' => {
                    if direction.0 != 1 {
                        return None;
                    }
                }
                b'<' => {
                    if direction.1 != -1 {
                        return None;
                    }
                }
                b'#' => {
                    // we will hit a wall, so change direction.
                    let mut found = false;
                    let excluded_neighbour = (
                        // don't step further back to where we came from
                        pos.0.checked_add_signed(-direction.0).unwrap(),
                        pos.1.checked_add_signed(-direction.1).unwrap(),
                    );

                    for neighbour_pos in self.neighbours(&pos).into_iter().flatten() {
                        if neighbour_pos == excluded_neighbour {
                            continue;
                        }

                        direction = (
                            isize::try_from(neighbour_pos.0).unwrap()
                                - isize::try_from(pos.0).unwrap(),
                            isize::try_from(neighbour_pos.1).unwrap()
                                - isize::try_from(pos.1).unwrap(),
                        );
                        found = true;
                        do_move = false;
                        break;
                    }

                    if !found {
                        // Hit a dead end (not sure if this is possible for my input)
                        return None;
                    }
                }
                _ => unreachable!(),
            }

            if do_move {
                pos = (
                    pos.0.checked_add_signed(direction.0).unwrap(),
                    pos.1.checked_add_signed(direction.1).unwrap(),
                );
                n_steps += 1;

                if vertices.contains(&pos) {
                    break;
                }
            }
        }

        Some((pos, n_steps))
    }
}
