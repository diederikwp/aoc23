use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;
use std::collections::VecDeque;
use std::hash::Hash;
use std::{error::Error, str::FromStr};

pub struct BrickPile(Vec<Brick>); // Vector is sorted by bottom z-coordinate ascending

impl FromStr for BrickPile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bricks = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Brick>, _>>()?;

        bricks.sort_by_key(|b| b.lfb.2);
        Self::drop_bricks(&mut bricks);

        Ok(BrickPile(bricks))
    }
}

impl BrickPile {
    pub fn n_bricks_destroyable(&self) -> u32 {
        let supported_by = self.find_all_supported_by();
        let load_bearing_bricks = self.find_load_bearing_bricks(&supported_by);

        u32::try_from(self.0.len() - load_bearing_bricks.len()).unwrap()
    }

    /// Sum over all bricks of how many bricks would fall if that brick were destroyed.
    pub fn sum_falling_count(&self) -> u32 {
        // Go over the graph in topologically sorted order, starting from the
        // bottom. For each brick, determine which other bricks would make it
        // fall (`tot_supported_by`). Also keep a running count of such bricks
        // in sum_falling.
        let supported_by = self.find_all_supported_by(); // immediate support, 1 step down
        let topsort = self.topological_sort(&supported_by);
        let mut tot_supported_by = HashMap::default(); // total support, all the way down
        let mut sum_falling = 0;

        for brick in topsort {
            let Some(below) = supported_by.get(brick) else {
                continue;
            };

            // Brick B supports brick A iff all supporting bricks of A are
            // themselves supported by B, so we need to find the intersection of
            // tot_supported_by among all supporting bricks of this brick.
            let mut intersection =
                set_intersection(below.iter().filter_map(|b| tot_supported_by.get(b)));
            if supported_by.get(brick).is_some_and(|sb| sb.len() == 1) {
                // If brick is immediately supported by just 1 brick, then that
                // brick should also be added.
                intersection.insert(supported_by[brick][0]);
            }
            sum_falling += intersection.len();
            tot_supported_by.insert(brick, intersection);
        }

        u32::try_from(sum_falling).unwrap()
    }

    /// Let the bricks fall down in z. Assumes `bricks` is sorted by bottom
    /// z-coordinate of the bricks.
    fn drop_bricks(bricks: &mut [Brick]) {
        // `bricks_argsort_top` contain indices into `bricks`, sorted by the top
        // z-coordinate of the bricks.
        let mut bricks_argsort_top: Vec<usize> = (0..bricks.len()).collect();
        bricks_argsort_top.sort_by_key(|&i| bricks[i].rbt.2);

        for idx in 0..bricks.len() {
            let brick = &bricks[idx];
            let mut new_z = 1;

            // Find new z-coordinate by iterating over all bricks whose tops are below this brick's bottom
            let idx_first_not_below =
                bricks_argsort_top.partition_point(|&i| bricks[i].rbt.2 < brick.lfb.2);
            for idx_brick_below in bricks_argsort_top[0..idx_first_not_below].iter().rev() {
                let brick_below = &bricks[*idx_brick_below];

                if brick.overlaps_x(brick_below) && brick.overlaps_y(brick_below) {
                    new_z = brick_below.rbt.2 + 1;
                    break;
                }
            }

            // Set new z-coordinate
            let brick = &mut bricks[idx];
            brick.rbt.2 -= brick.lfb.2 - new_z;
            brick.lfb.2 = new_z;

            // Reorder bricks_argsort_top to keep them sorted
            bricks_argsort_top.sort_by_key(|&i| bricks[i].rbt.2);
        }

        bricks.sort_by_key(|brick| brick.lfb.2);
    }

    fn find_all_supported_by(&self) -> HashMap<&Brick, Vec<&Brick>> {
        let mut supported_by = HashMap::default();

        for (idx, brick) in self.0.iter().enumerate() {
            for brick_above in &self.0[(idx + 1)..] {
                if brick_above.lfb.2 > brick.rbt.2 + 1 {
                    break; // this brick_above and following cannot be supported by brick
                }

                if brick_above.overlaps_x(brick) && brick_above.overlaps_y(brick) {
                    supported_by
                        .entry(brick_above)
                        .or_insert(Vec::new())
                        .push(brick);
                }
            }
        }
        supported_by
    }

    fn find_all_supporting<'a>(
        &'a self,
        supported_by: &HashMap<&'a Brick, Vec<&'a Brick>>,
    ) -> HashMap<&'a Brick, HashSet<&'a Brick>> {
        let mut supporting = HashMap::default();
        for (&brick, supporting_bricks) in supported_by {
            for &supporting_brick in supporting_bricks {
                supporting
                    .entry(supporting_brick)
                    .or_insert(HashSet::default())
                    .insert(brick);
            }
        }

        supporting
    }

    fn find_load_bearing_bricks<'a>(
        &'a self,
        supported_by: &HashMap<&'a Brick, Vec<&'a Brick>>,
    ) -> HashSet<&'a Brick> {
        let mut load_bearing_bricks = HashSet::default();
        for supporting_bricks in supported_by.values() {
            if supporting_bricks.len() == 1 {
                load_bearing_bricks.insert(supporting_bricks[0]);
            }
        }
        load_bearing_bricks
    }

    /// Find any topological ordering, starting from the bottom bricks
    fn topological_sort<'a>(
        &'a self,
        supported_by: &HashMap<&'a Brick, Vec<&'a Brick>>,
    ) -> Vec<&'a Brick> {
        // using Kahn's algorithm. Assuming acyclic graph.
        let mut n_unvisited_below: HashMap<&Brick, usize> = self
            .0
            .iter()
            .map(|b| (b, supported_by.get(b).map(|sb| sb.len()).unwrap_or(0)))
            .collect();
        let bottom_bricks: Vec<_> = self
            .0
            .iter()
            .filter(|&b| supported_by.get(b).is_none())
            .collect();
        let mut queue = VecDeque::from(bottom_bricks);
        let mut topsort = Vec::with_capacity(self.0.len());
        let supporting = self.find_all_supporting(supported_by);

        while let Some(brick) = queue.pop_front() {
            topsort.push(brick);

            let Some(above) = supporting.get(brick) else {
                continue;
            };

            for brick_above in above {
                let n = n_unvisited_below.get_mut(brick_above).unwrap();
                *n -= 1;
                if *n == 0 {
                    queue.push_back(brick_above);
                }
            }
        }

        topsort
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Brick {
    lfb: (u32, u32, u32), // Left Front Bottom (x, y, z)
    rbt: (u32, u32, u32), // Right Back Top (x, y, z)
}

impl FromStr for Brick {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lfb_str, rbt_str) = s.split_once('~').ok_or::<String>("Missing '~'".into())?;
        let mut lfb_coords = lfb_str.split(',');
        let mut rbt_coords = rbt_str.split(',');

        let lfb = (
            lfb_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
            lfb_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
            lfb_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
        );
        if lfb_coords.next().is_some() {
            return Err("Too many coords".into());
        }

        let rbt = (
            rbt_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
            rbt_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
            rbt_coords
                .next()
                .ok_or::<String>("Missing coord".into())?
                .parse()?,
        );
        if rbt_coords.next().is_some() {
            return Err("Too many coords".into());
        }

        if lfb.0 > rbt.0 || lfb.1 > rbt.1 || lfb.2 > rbt.2 {
            return Err("Left coord may not exceed right coord".into());
        }
        if lfb.2 < 1 {
            return Err("Z-coordinates should be >= 1".into());
        }

        Ok(Brick { lfb, rbt })
    }
}

impl Brick {
    fn overlaps_x(&self, other: &Brick) -> bool {
        self.lfb.0 <= other.rbt.0 && other.lfb.0 <= self.rbt.0
    }

    fn overlaps_y(&self, other: &Brick) -> bool {
        self.lfb.1 <= other.rbt.1 && other.lfb.1 <= self.rbt.1
    }
}

fn set_intersection<'a, I, T>(sets: I) -> HashSet<T>
where
    I: IntoIterator<Item = &'a HashSet<T>>,
    I::IntoIter: Clone,
    T: 'a + Eq + PartialEq + Hash + Clone,
{
    let iter = sets.into_iter();

    // First find the smallest set, because it is most efficient to iterate over
    // that.
    let smallest = iter.clone().min_by_key(|s| s.len());
    let Some(smallest) = smallest else {
        return HashSet::default();
    };

    let mut intersection = HashSet::default();
    for item in smallest {
        if iter.clone().all(|s| s.contains(item)) {
            intersection.insert(item.clone());
        }
    }

    intersection
}
