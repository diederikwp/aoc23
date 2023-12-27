use std::{str::FromStr, error::Error, mem};
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

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
        let mut supported_by = HashMap::default();

        for (idx, brick) in self.0.iter().enumerate() {
            for brick_above in &self.0[(idx + 1)..] {
                if brick_above.lfb.2 > brick.rbt.2 + 1 {
                    break;  // this brick_above and following cannot be supported by brick
                }

                if brick_above.overlaps_x(brick) && brick_above.overlaps_y(brick) {
                    supported_by.entry(brick_above).or_insert(Vec::new()).push(brick);
                }
            }
        }
        
        let mut bricks_to_keep = HashSet::default();
        for supporting_bricks in supported_by.values() {
            if supporting_bricks.len() == 1 {
                bricks_to_keep.insert(supporting_bricks[0]);
            }
        }

        u32::try_from(self.0.len() - bricks_to_keep.len()).unwrap()
    }

    /// Let the bricks fall down in z. Assumes `bricks` is sorted by bottom
    /// z-coordinate of the bricks.
    fn drop_bricks(bricks: &mut [Brick]) {
        let mut bricks_sorted_top = bricks.to_vec();
        bricks_sorted_top.sort_by_key(|b| b.rbt.2);

        for idx in 0..bricks.len() {
            let brick = &bricks[idx];
            let mut new_z = 1;
            let mut bricks_skipped = 0;

            // Find new z-coordinate by iterating over all bricks whose tops are below this brick's bottom
            let idx_first_below = match bricks_sorted_top.binary_search_by(|other| other.rbt.2.cmp(&(brick.lfb.2 - 1))) {
                Ok(idx_found) => idx_found + 1,
                Err(idx_insert) => idx_insert,
            };
            for brick_below in bricks_sorted_top[0..idx_first_below].iter().rev() {
                if brick.overlaps_x(brick_below) && brick.overlaps_y(brick_below) {
                    new_z = brick_below.rbt.2 + 1;
                    break;
                } else {
                    bricks_skipped += 1;
                }
            }

            // Set new z-coordinate
            let idx_top = bricks_sorted_top.binary_search_by(|other| other.rbt.2.cmp(&(brick.rbt.2))).unwrap();
            let brick_top = &mut bricks_sorted_top[idx_top];
            let brick = &mut bricks[idx];

            brick.rbt.2 -= brick.lfb.2 - new_z;
            brick.lfb.2 = new_z;
            brick_top.rbt = brick.rbt;
            brick_top.lfb = brick.lfb;

            // Reorder bricks_sorted_top to keep them sorted
            bricks_sorted_top.sort_by_key(|b| b.rbt.2);
            // for dest_idx in ((idx_top - bricks_skipped)..idx_top).rev() {
            //     let (left, right) = bricks_sorted_top.split_at_mut(dest_idx + 1);
            //     mem::swap(&mut left[left.len() - 1], &mut right[0]);
            // }
        }

        bricks.sort_by_key(|brick| brick.lfb.2);
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
            lfb_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
            lfb_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
            lfb_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
        );
        if lfb_coords.next().is_some() {
            return Err("Too many coords".into());
        }

        let rbt = (
            rbt_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
            rbt_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
            rbt_coords.next().ok_or::<String>("Missing coord".into())?.parse()?,
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
