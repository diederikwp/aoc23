use std::{error::Error, iter::zip, ops::Range, str::FromStr};

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

    pub fn find_perfect_throw_velocity_and_position(&self) -> (i64, i64, i64, i64, i64, i64) {
        let n = self.0.len();
        let n = n.min(4); // just using the first 4 stones should be enough

        // Start with only x and y. Brute force check all x and y velocities of
        // stone to throw. Transform the other stones to the frame of reference
        // of the thrown stone, so that their paths must all intersect in 1
        // point.
        let mut spiral = SpiralCoords::new();
        let (vthrow_x, vthrow_y, x_throw, _y_throw) = loop {
            let (vx, vy) = spiral.next();

            let mut intersections = zip(0..n, 1..n).map(|(i, j)| {
                let stone_i = &self.0[i].add_velocity(&(-vx, -vy, 0));
                let stone_j = &self.0[j].add_velocity(&(-vx, -vy, 0));
                stone_i.intersection_at_integer_xy(stone_j)
            });

            // Check if the first 2 intersect
            let Some(coords) = intersections.next().unwrap() else {
                continue;
            };

            // Check if all others intersect in the same place
            if intersections.all(|icn| icn.is_some_and(|xy| xy == coords)) {
                break (vx, vy, coords.0, coords.1);
            }
        };

        // find vz that makes all stones intersect in z as well
        let mut i = 0;
        let vthrow_z = loop {
            i += 1;
            let vz = (i / 2) * ((i % 2) * -2 + 1); // 0, 1, -1, 2, -2, 3, ...

            let mut z_intersections = (0..n).map(|i| {
                let stone = &self.0[i].add_velocity(&(-vthrow_x, -vthrow_y, -vz));
                let t = if stone.vx != 0 {
                    (x_throw - stone.x) / stone.vx
                } else {
                    0
                };
                t * stone.vz + stone.z
            });

            let first = z_intersections.next().unwrap();
            if z_intersections.all(|z| z == first) {
                break vz;
            }
        };

        // coordinate of intersection
        let stone0 = &self.0[0].add_velocity(&(-vthrow_x, -vthrow_y, -vthrow_z));
        let t0 = if stone0.vx != 0 {
            (x_throw - stone0.x) / stone0.vx
        } else {
            0
        };
        let (x_throw, y_throw, z_throw) = (
            stone0.x + t0 * stone0.vx,
            stone0.y + t0 * stone0.vy,
            stone0.z + t0 * stone0.vz,
        );

        (x_throw, y_throw, z_throw, vthrow_x, vthrow_y, vthrow_z)
    }
}

struct SpiralCoords {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    radius: i64,
}

impl SpiralCoords {
    fn new() -> Self {
        SpiralCoords {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
            radius: 0,
        }
    }

    fn next(&mut self) -> (i64, i64) {
        let ret_x = self.x;
        let ret_y = self.y;

        // spiral round (0, 0). Turn on the corners.
        self.x += self.dx;
        self.y += self.dy;

        if self.dx == 1 && self.x == self.radius + 1 {
            self.radius += 1;

            // dx -> dy, dy -> -dx, i.e. rotate 90 degrees
            std::mem::swap(&mut self.dx, &mut self.dy);
            self.dy *= -1;
        } else if (self.dx == -1 && self.x == -self.radius) || self.dy * self.y == self.radius {
            std::mem::swap(&mut self.dx, &mut self.dy);
            self.dy *= -1;
        }

        (ret_x, ret_y)
    }
}

struct HailStone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl FromStr for HailStone {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s.split_once(" @ ").ok_or("Invalid syntax")?;
        let mut pos_iter = pos_str.split(", ");
        let mut vel_iter = vel_str.split(", ");

        let x = pos_iter.next().ok_or("Missing position")?.trim().parse()?;
        let y = pos_iter.next().ok_or("Missing position")?.trim().parse()?;
        let z = pos_iter.next().ok_or("Missing position")?.trim().parse()?;

        let vx = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;
        let vy = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;
        let vz = vel_iter.next().ok_or("Missing velocity")?.trim().parse()?;

        if pos_iter.next().is_some() {
            return Err("Too many positions".into());
        }
        if vel_iter.next().is_some() {
            return Err("Too many velocities".into());
        }
        if vx == 0 || vy == 0 || vz == 0 {
            return Err("Velocity x,y,z components may not be 0".into());
        }

        Ok(HailStone {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        })
    }
}
type XYTIntersection = (f64, f64, f64, f64); // (x, y, t0, t1)

impl HailStone {
    fn slope(&self) -> f64 {
        self.vy as f64 / self.vx as f64
    }

    fn intercept(&self) -> f64 {
        self.y as f64 - self.x as f64 * self.slope()
    }

    fn speed_squared_xy(&self) -> i64 {
        self.vx * self.vx + self.vy * self.vy
    }

    fn is_parallel_to_xy(&self, other: &HailStone) -> bool {
        let dotprod = self.vx * other.vx + self.vy * other.vy;
        dotprod * dotprod == self.speed_squared_xy() * other.speed_squared_xy()
    }

    fn add_velocity(&self, velocity: &(i64, i64, i64)) -> Self {
        HailStone {
            vx: self.vx + velocity.0,
            vy: self.vy + velocity.1,
            vz: self.vz + velocity.2,
            ..*self
        }
    }

    /// If self and other intersect at integer (x, y) coordinates, return them.
    /// If they don't intersect at integer coordinates or don't intersect at
    /// all, return None.
    fn intersection_at_integer_xy(&self, other: &HailStone) -> Option<(i64, i64)> {
        if self.is_parallel_to_xy(other) {
            return None;
        }

        let dx = other.x - self.x;
        let dy = other.y - self.y;

        // Now it should hold that for integer n and m
        // dx == n * self.vx - m * other.vx
        // dy == m * self.vy - m * other.vy
        // from which it follows that

        let (nom_n, denom_n); // n == nom_n / denom_n
        if other.vx == 0 {
            nom_n = dx;
            denom_n = self.vx;
        } else {
            nom_n = dx * other.vy - dy * other.vx;
            denom_n = self.vx * other.vy - other.vx * self.vy;
        }
        if nom_n % denom_n != 0 {
            return None;
        }
        let n = nom_n / denom_n;

        let (nom_m, denom_m); // m == nom_m / denom_m
        if self.vx == 0 {
            nom_m = -dx;
            denom_m = other.vx;
        } else {
            nom_m = dy * self.vx - dx * self.vy;
            denom_m = other.vx * self.vy - self.vx * other.vy;
        }
        if nom_m % denom_m != 0 {
            return None;
        }
        let _m = nom_m / denom_m;

        let x = self.x + n * self.vx; // == other.x + m * other.vx
        let y = self.y + n * self.vy; // == other.y + m * other.vy
        Some((x, y))
    }

    fn xyt_intersection(&self, other: &HailStone) -> Option<XYTIntersection> {
        // No intersection if one of the stones is not moving or they are
        // parallel, and not moving is impossible as validated when parsing the
        // input.
        if self.is_parallel_to_xy(other) {
            return None;
        }

        let x_intersect = (other.intercept() - self.intercept()) / (self.slope() - other.slope());
        let y_intersect = self.intercept() + x_intersect * self.slope();

        let t_intersect_self = (y_intersect - self.y as f64) / self.vy as f64;
        let t_intersect_other = (y_intersect - other.y as f64) / other.vy as f64;

        Some((
            x_intersect,
            y_intersect,
            t_intersect_self,
            t_intersect_other,
        ))
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
