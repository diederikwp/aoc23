use std::collections::VecDeque;
use std::{error::Error, str::FromStr};

use ndarray::{Array, Array2};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use self::bitmap::BitMap64;

type Pos = (usize, usize);

pub struct Map {
    grid: Grid,
    vertex2idx: HashMap<Pos, u32>,
    idx2vertex: HashMap<u32, Pos>,
    edges_out: HashMap<u32, Vec<(u32, u32)>>, // key: idx_from, value: pairs of (idx_to, distance)
    edges_in: HashMap<u32, Vec<(u32, u32)>>,
    edges_undirected: HashMap<u32, Vec<(u32, u32)>>,
}

impl FromStr for Map {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Grid = s.parse()?;
        Ok(Self::from_grid(grid))
    }
}

impl Map {
    pub fn print_graphviz(&self) {
        println!("digraph G {{");

        for (idx_vx_from, outgoing_edges) in &self.edges_out {
            for (idx_vx_to, dist) in outgoing_edges {
                let vx_from = self.idx2vertex[idx_vx_from];
                let vx_to = self.idx2vertex[idx_vx_to];
                println!("\"{vx_from:?}\" -> \"{vx_to:?}\" [ label=\"{dist}\" ];")
            }
        }

        println!("}}");
    }

    pub fn longest_path_len_directed(&self) -> u32 {
        // Assumption: graph is acyclic (a DAG)
        let topsort = self.topological_sort();
        let mut max_total_dist: HashMap<u32, u32> = HashMap::default();
        max_total_dist.insert(self.entrance_idx(), 0);

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

        max_total_dist[&self.exit_idx()]
    }

    pub fn longest_path_len_undirected(&self) -> u32 {
        // Just brute-force search all paths using DFS. I couldn't think of or
        // find a better algorithm.

        // Stack frames contain (vertex, distance from entrance,
        // visited_vertices). For the visited_vertices we use a BitMap64 as
        // opposed to a HashMap to reduce the amount of memcopying.  This limits
        // the number of vertices to 64 which is ok (my puzzle input has ~30).
        // This works because we store node indexes (not positions) which are
        // consecutive integers.

        let exit = self.exit_idx();
        let mut longest_dist = 0;
        let mut stack = vec![(self.entrance_idx(), 0, BitMap64::new())];

        while let Some((vx, total_dist, visited)) = stack.pop() {
            if vx == exit {
                longest_dist = u32::max(longest_dist, total_dist);
                continue;
            }

            let Some(edges) = self.edges_undirected.get(&vx) else {
                continue;
            };
            for (neighbour, dist) in edges {
                if visited.get(*neighbour) {
                    continue;
                }

                let mut new_visited = visited.clone();
                new_visited.set_unchecked(vx);
                stack.push((*neighbour, total_dist + dist, new_visited));
            }
        }

        longest_dist
    }

    fn entrance_idx(&self) -> u32 {
        self.vertex2idx[&self.grid.entrance()]
    }

    fn exit_idx(&self) -> u32 {
        self.vertex2idx[&self.grid.exit()]
    }

    fn from_grid(grid: Grid) -> Self {
        let vertex2idx = grid.find_vertices();
        let edges_out = grid.find_edges_out(&vertex2idx);
        let idx2vertex = Self::invert_vertex2idx(&vertex2idx);
        let edges_in = Self::make_edges_in_from_edges_out(&edges_out);
        let edges_undirected = Self::make_undirected_edges_from_edges_out(&edges_out);
        Map {
            grid,
            vertex2idx,
            idx2vertex,
            edges_out,
            edges_in,
            edges_undirected,
        }
    }

    fn invert_vertex2idx(vertex2idx: &HashMap<Pos, u32>) -> HashMap<u32, Pos> {
        vertex2idx.iter().map(|(pos, idx)| (*idx, *pos)).collect()
    }

    fn make_edges_in_from_edges_out(
        edges_out: &HashMap<u32, Vec<(u32, u32)>>,
    ) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut edges_in: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();
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

    fn make_undirected_edges_from_edges_out(
        edges_out: &HashMap<u32, Vec<(u32, u32)>>,
    ) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut edges: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();

        for (&vx, outgoing_edges) in edges_out {
            for &(neighbour, distance) in outgoing_edges {
                edges.entry(vx).or_default().push((neighbour, distance));
                edges.entry(neighbour).or_default().push((vx, distance));
            }
        }

        edges
    }

    fn topological_sort(&self) -> Vec<u32> {
        // using DFS. Assuming acyclic graph.
        let mut topsort = VecDeque::new();
        let mut visited = HashSet::default();
        let mut stack = vec![(self.entrance_idx(), false)];

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

    fn find_vertices(&self) -> HashMap<Pos, u32> {
        let height = self.0.shape()[0];
        let width = self.0.shape()[1];

        let mut vertices = HashMap::default();
        vertices.insert(self.entrance(), 0);
        vertices.insert(self.exit(), 1);

        let mut idx = 2;
        for y in 1..(height - 1) {
            for x in 1..(width - 1) {
                if self.0[(y, x)] == b'#' {
                    continue;
                }

                let n_neighbours = self.neighbours(&(y, x)).iter().flatten().count();
                if n_neighbours > 2 {
                    vertices.insert((y, x), idx);
                    idx += 1;
                }
            }
        }

        vertices
    }

    fn find_edges_out(&self, vertices: &HashMap<Pos, u32>) -> HashMap<u32, Vec<(u32, u32)>> {
        let mut edges: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();

        for (vx, idx) in vertices {
            for neighbour_pos in self.neighbours(vx).into_iter().flatten() {
                let direction = (
                    isize::try_from(neighbour_pos.0).unwrap() - isize::try_from(vx.0).unwrap(),
                    isize::try_from(neighbour_pos.1).unwrap() - isize::try_from(vx.1).unwrap(),
                );

                if let Some((target_vx, steps)) = self.walk_to_next_vertex(vx, direction, vertices)
                {
                    let target_idx = vertices[&target_vx];
                    edges.entry(*idx).or_default().push((target_idx, steps));
                }
            }
        }

        edges
    }

    fn walk_to_next_vertex(
        &self,
        start_pos: &Pos,
        start_direction: (isize, isize),
        vertices: &HashMap<Pos, u32>,
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

                if vertices.contains_key(&pos) {
                    break;
                }
            }
        }

        Some((pos, n_steps))
    }
}

pub mod bitmap {
    #[derive(Clone, Default)]
    pub struct BitMap64(u64);

    impl BitMap64 {
        pub fn new() -> Self {
            BitMap64(0)
        }

        pub fn get(&self, idx: u32) -> bool {
            self.0 & (1 << idx) != 0
        }

        pub fn set_unchecked(&mut self, idx: u32) {
            // Passing idx > 63 would be a mistake, but this is ignored without
            // setting anything.
            self.0 |= 1 << idx
        }
    }
}
