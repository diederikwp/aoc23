use std::{error::Error, ops::Range, str::FromStr};

pub struct Hail(Vec<HailStone>);

impl FromStr for Hail {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<HailStone>, _>>()?;

        Ok(Hail(stones))
    }
}

impl Hail {
    pub fn count_intersections_within_xy(&self, x_range: &Range<f64>, y_range: &Range<f64>) -> u32 {
        let mut n = 0;

        for i in 0..self.0.len() {
            for j in (i + 1)..self.0.len() {
                let stone_i = &self.0[i];
                let stone_j = &self.0[j];

                if let Some((x, y, t1, t2)) = stone_i.xyt_intersection(stone_j) {
                    if x >= x_range.start
                        && x <= x_range.end
                        && y >= y_range.start
                        && y <= y_range.end
                        && t1 >= 0.0
                        && t2 >= 0.0
                    {
                        n += 1;
                    }
                }
            }
        }

        n
    }
}

struct HailStone {
    x: f64,
    y: f64,
    // z: u64,
    vx: f64,
    vy: f64,
    // vz: u64
}

impl FromStr for HailStone {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s.split_once(" @ ").ok_or("Invalid syntax")?;
        let mut pos_iter = pos_str.split(", ");
        let mut vel_iter = vel_str.split(", ");

        let x = pos_iter.next().ok_or("Missing position")?.trim().parse()?;
        let y = pos_iter.next().ok_or("Missing position")?.trim().parse()?;
        let _z: f64 = pos_iter.next().ok_or("Missing position")?.trim().parse()?;

        let vx = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;
        let vy = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;
        let _vz: f64 = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;

        if pos_iter.next().is_some() {
            return Err("Too many positions".into());
        }
        if vel_iter.next().is_some() {
            return Err("Too many velocities".into());
        }

        // Ok(HailStone { x, y, z, vx, vy, vz })
        Ok(HailStone { x, y, vx, vy })
    }
}

impl HailStone {
    fn slope(&self) -> f64 {
        self.vy / self.vx
    }

    fn intercept(&self) -> f64 {
        self.y - self.x * self.slope()
    }

    fn xyt_intersection(&self, other: &HailStone) -> Option<(f64, f64, f64, f64)> {
        if self.vx * other.vy == other.vx * self.vy {
            return None; // The paths are parallel, so they'll never intersect
        }

        let x_intersect = (other.intercept() - self.intercept()) / (self.slope() - other.slope());
        let y_intersect = self.intercept() + x_intersect * self.slope();
        let t1_intersect = (x_intersect - self.x) / self.vx;
        let t2_intersect = (x_intersect - other.x) / other.vx;
        Some((x_intersect, y_intersect, t1_intersect, t2_intersect))
    }
}

#[cfg(test)]
mod tests {
    use crate::Day;

    use super::*;

    #[test]
    fn test_count_intersections_within_xy() {
        let input = crate::template::read_file("examples", Day::new(24).unwrap());
        let hail: Hail = input.parse().unwrap();
        let range = 7.0f64..27f64;
        let count = hail.count_intersections_within_xy(&range, &range);

        assert_eq!(count, 2);
    }
}
