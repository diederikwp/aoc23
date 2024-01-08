use std::{error::Error, str::FromStr};

use rand::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

use self::union_find::UnionFind;

pub struct Wiring {
    idxs: HashMap<usize, String>,
    connections: Vec<(usize, usize)>,
}

impl FromStr for Wiring {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = HashSet::<String>::default();
        let mut str_cnxns = HashMap::<String, Vec<String>>::default();

        for line in s.lines() {
            let (from_node, to) = line.split_once(": ").ok_or("Missing separator")?;
            let to_nodes: Vec<String> = to.split_whitespace().map(String::from).collect();
            components.insert(from_node.into());
            components.extend(to_nodes.iter().cloned());
            str_cnxns.insert(from_node.into(), to_nodes);
        }

        let str2idx: HashMap<String, usize> = components
            .iter()
            .enumerate()
            .map(|(idx, str)| (str.to_string(), idx))
            .collect();

        let mut idx_cnxns = Vec::new();
        for component in components {
            if !str_cnxns.contains_key(&component) {
                continue;
            }

            let idx_from = str2idx[&component];
            let idxs_to: Vec<usize> = str_cnxns[&component].iter().map(|s| str2idx[s]).collect();
            for idx_to in idxs_to {
                idx_cnxns.push((idx_from, idx_to));
            }
        }

        let idx2string: HashMap<usize, String> = str2idx.into_iter().map(|(s, i)| (i, s)).collect();

        Ok(Wiring {
            idxs: idx2string,
            connections: idx_cnxns,
        })
    }
}

impl Wiring {
    pub fn min_cut(&self) -> MinCut {
        // Repeated Karger's algorithm and return best (best being the cut with
        // the minimum number of edges; there could in general be multiple
        // different such minimum cuts, though in the puzzle input probably
        // not).

        // for c * n_choose_2 * ln(n) trials, the success probability is at
        // least 1 - (1 / n ^ c), where n is the number of nodes. However, we
        // can just search until we have a minimum cut of 3 since it is already
        // given that this is the minimum.
        let mut rng = StdRng::seed_from_u64(42);
        let mut best_cut = self.kargers_algorithm(&mut rng);
        while best_cut.edges.len() > 3 {
            let min_cut = self.kargers_algorithm(&mut rng);

            let n_edge = min_cut.edges.len();
            let n_edge_best = best_cut.edges.len();
            if n_edge < n_edge_best {
                best_cut = min_cut;
            }
        }

        best_cut
    }

    fn kargers_algorithm(&self, rng: &mut impl Rng) -> MinCut {
        assert!(
            self.idxs.len() > 1,
            "Can't cut a graph of less than 2 nodes"
        );

        let mut edges = self.connections.clone();
        edges.shuffle(rng);
        let edges = edges;

        let mut super_nodes: UnionFind<usize> = self.idxs.keys().cloned().collect();
        let mut n = 0;
        while super_nodes.n_sets() > 2 {
            let (from, to) = &edges[n];
            super_nodes.union(from, to);
            n += 1;
        }

        let remaining_edges = edges[n..]
            .iter()
            .filter(|(node_a, node_b)| {
                let root_a = super_nodes.find_root(node_a).unwrap();
                let root_b = super_nodes.find_root(node_b).unwrap();
                root_a != root_b
            }) // ignore edges within the same component
            .cloned()
            .collect();

        MinCut {
            edges: remaining_edges,
            components: super_nodes,
        }
    }
}

pub struct MinCut {
    edges: Vec<(usize, usize)>, // The edges between the 2 components, e.g. across the cut
    components: UnionFind<usize>,
}

impl MinCut {
    pub fn size(&self) -> usize {
        self.edges.len()
    }

    pub fn component_sizes(&self) -> (usize, usize) {
        let (node_a, node_b) = &self.edges[0];
        let root_a = self.components.find_root(node_a).unwrap();
        let root_b = self.components.find_root(node_b).unwrap();

        (
            self.components.size_of_set(root_a).unwrap(),
            self.components.size_of_set(root_b).unwrap(),
        )
    }
}

/// Union-Find container data structure
pub mod union_find {
    use rustc_hash::FxHashMap as HashMap;
    use std::hash::Hash;

    // Invariants:
    // - idx2elem holds only valid pointers to keys of elem2idx. If elem2idx
    //   would be moved/mutated without updating idx2elem, this could lead to
    //   dereferencing an invalid pointer.
    // - the values of elem2idx are unique and in the range 0..elems.len().
    // - the values of parents are unique and in the range 0..elems.len().
    // - elem2idx, idx2elem, parents and sizes have equal length.
    pub struct UnionFind<T> {
        elem2idx: HashMap<T, usize>, // key: element, value: index into elems, parents and sizes
        idx2elem: Vec<*const T>,     // translates index back to element. Points into elem2idx.
        parents: Vec<usize>,         // parent idx of each node idx (equal to itself for root)
        sizes: Vec<usize>,           // size of subtree (including node itself) of each node
        n_sets: usize,
    }

    impl<T> FromIterator<T> for UnionFind<T>
    where
        T: Eq + Hash,
    {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let elem2idx: HashMap<T, usize> = iter
                .into_iter()
                .enumerate()
                .map(|(idx, elem)| (elem, idx))
                .collect();
            let idx2elem = Vec::new();
            let parents = (0..elem2idx.len()).collect();
            let sizes = vec![1; elem2idx.len()];
            let n_sets = elem2idx.len();

            let mut uf = UnionFind {
                elem2idx,
                idx2elem,
                parents,
                sizes,
                n_sets,
            };

            // From now on, idx2elem has its location inside uf, and mutating a
            // field of uf will not move uf. So at this point we can safely
            // create the idx2elem vec.
            let mut idx2elem: Vec<(usize, *const T)> = uf
                .elem2idx
                .iter()
                .map(|(elem, &idx)| (idx, elem as *const T))
                .collect();
            idx2elem.sort_by_key(|x| x.0);

            let idx2elem: Vec<*const T> = idx2elem.into_iter().map(|(_i, p)| p).collect();
            uf.idx2elem = idx2elem;

            uf
        }
    }

    impl<T> UnionFind<T>
    where
        T: Eq + Hash + PartialEq,
    {
        /// Find this element and (if found) return the root element of the set
        /// it is in. Unlike find_root_shorten, this requires no mutable access.
        pub fn find_root(&self, elem: &T) -> Option<&T> {
            let root_idx = self.find_root_idx(elem)?;
            unsafe { Some(&*self.idx2elem[root_idx]) }
        }

        fn find_root_idx(&self, elem: &T) -> Option<usize> {
            let mut walk = *self.elem2idx.get(elem)?;
            loop {
                let parent = self.parents[walk];
                if walk == parent {
                    return Some(walk);
                }
                walk = parent;
            }
        }

        /// Find this element and (if found) return the root element of the set
        /// it is in. Also shorten the trees for better efficiency of future
        /// operations.
        pub fn find_root_shorten(&mut self, elem: &T) -> Option<&T> {
            let root_idx = self.find_root_idx_shorten(elem)?;
            unsafe { Some(&*self.idx2elem[root_idx]) }
        }

        fn find_root_idx_shorten(&mut self, elem: &T) -> Option<usize> {
            // First pass: find root.
            let start_idx = *self.elem2idx.get(elem)?;
            let mut walk = start_idx;
            loop {
                let parent = self.parents[walk];
                if walk == parent {
                    break;
                }
                walk = parent;
            }
            let root_idx = walk;

            // Second pass: move all children directly under the root.
            let mut walk = start_idx;
            while walk != root_idx {
                self.parents[walk] = root_idx;
                self.sizes[walk] = 1;
                walk = self.parents[walk];
            }

            Some(walk)
        }

        /// Union the sets containing each element, returning None if they were
        /// already part of the same set or if either one is not in the
        /// container, and returning the root element of the unioned set if the
        /// merge was successful.
        pub fn union(&mut self, elem1: &T, elem2: &T) -> Option<&T> {
            let root_idx1 = self.find_root_idx_shorten(elem1)?;
            let root_idx2 = self.find_root_idx_shorten(elem2)?;
            if root_idx1 == root_idx2 {
                return None;
            }

            let result_idx = if self.sizes[root_idx1] > self.sizes[root_idx2] {
                self.parents[root_idx2] = root_idx1;
                self.sizes[root_idx1] += self.sizes[root_idx2];
                self.n_sets -= 1;
                root_idx1
            } else {
                self.parents[root_idx1] = root_idx2;
                self.sizes[root_idx2] += self.sizes[root_idx1];
                self.n_sets -= 1;
                root_idx2
            };
            unsafe { Some(&*self.idx2elem[result_idx]) }
        }

        /// The size of the set that elem is part of.
        pub fn size_of_set(&self, elem: &T) -> Option<usize> {
            let root_idx = self.find_root_idx(elem)?;
            Some(self.sizes[root_idx])
        }

        pub fn n_sets(&self) -> usize {
            self.n_sets
        }

        pub fn len(&self) -> usize {
            self.elem2idx.len()
        }

        pub fn is_empty(&self) -> bool {
            self.elem2idx.len() == 0
        }
    }
}
