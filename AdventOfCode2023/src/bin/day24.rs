/// Advent of Code 2023, Day 24
/// Copyright 2023 by Alex Utter

use aocfetch;
use core::cmp::max;

// An X/Y/Z triplet for a position or a velocity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Xyz<T>(T, T, T);

impl<T> Xyz<T>
    where T: std::marker::Copy + 'static,
          T: std::ops::Add<Output=T>,
          T: std::ops::Mul<Output=T>,
          T: std::ops::Sub<Output=T>,
{
    fn as_<U>(&self) -> Xyz<U>
        where T: num::cast::AsPrimitive<U>,
              U: std::marker::Copy + 'static,
    {
        Xyz(self.0.as_(), self.1.as_(), self.2.as_())
    }

    fn add(&self, other:&Self) -> Self {
        Xyz(self.0+other.0, self.1+other.1, self.2+other.2)
    }

    fn sub(&self, other:&Self) -> Self {
        Xyz(self.0-other.0, self.1-other.1, self.2-other.2)
    }

    fn mul(&self, scale:T) -> Self {
        Xyz(self.0*scale, self.1*scale, self.2*scale)
    }
}


// An object with an initial position and a velocity.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Hail {
    pos: Xyz<i64>,
    vel: Xyz<i64>,
}

impl Hail {
    fn new(input: &str) -> Self {
        let tok: Vec<i64> = input.split(&[',', '@'])
            .map(|s| s.trim().parse::<i64>().unwrap()).collect();
        Hail {
            pos: Xyz(tok[0], tok[1], tok[2]),
            vel: Xyz(tok[3], tok[4], tok[5]),
        }
    }

    // Return maximum absolute value of velocity along any axis.
    fn magvel(&self) -> i64 {
        max(self.vel.0.abs(), max(self.vel.1.abs(), self.vel.2.abs()))
    }

    // Considering X/Y coordinates only, find the elapsed time for
    // this object to cross the other's forward trajectory, ignoring
    // the time at which the other object reaches that point.
    // Returns tuple with (numerator, denominator), time = num/den.
    fn cross_xy(&self, other: &Hail) -> Option<(i64,i64)> {
        let dp0 = other.vel.0 * (self.pos.1 - other.pos.1)
                - other.vel.1 * (self.pos.0 - other.pos.0);
        let dp1 = self.vel.1 * (other.pos.0 - self.pos.0)
                - self.vel.0 * (other.pos.1 - self.pos.1);
        let dv = self.vel.0 * other.vel.1 - self.vel.1 * other.vel.0;
        if dv != 0 && dp0 * dv > 0 && dp1 * dv > 0 {
            return Some((dp0, dv));
        } else {
            return None;
        }
    }

    // Predict the position of this object at a given time.
    fn predict<T>(&self, t: T) -> Xyz<T>
        where i64: num::cast::AsPrimitive<T>,
              T: std::marker::Copy + 'static,
              T: std::ops::Add<Output=T>,
              T: std::ops::Mul<Output=T>,
              T: std::ops::Sub<Output=T>,
    {
        return self.pos.as_().add(&self.vel.as_().mul(t));
    }

    // Given a velocity, plot a trajectory of possible starting positions.
    fn intercept(&self, vel: &Xyz<i64>) -> Self {
        Hail { pos:self.pos, vel:self.vel.sub(vel) }
    }

    // Given another trajectory, find the intersection point if any.
    // Set flag to check for matching time; otherwise any crossing.
    fn intersect(&self, other: &Hail) -> Option<Xyz<i64>> {
        let dp = other.pos.sub(&self.pos);
        let dv = self.vel.sub(&other.vel);
        if dp.0 * dv.0 > 0 &&
           dp.0 * dv.1 == dp.1 * dv.0 &&
           dp.0 * dv.2 == dp.2 * dv.0 {
            let t = dp.0 / dv.0;
            let p0 = self.predict(t);
            let p1 = self.predict(t);
            if p0 == p1 {return Some(p0);}
        }
        return None;
    }
}

struct HailStorm {
    hail: Vec<Hail>,
}

impl HailStorm {
    fn new(input:&str) -> Self {
        HailStorm { hail:input.trim().lines().map(Hail::new).collect() }
    }

    // Part one solution: Count the number of in-bounds pseudo-crossings.
    fn part1(&self, xmin:i64, xmax:i64) -> usize {
        let mut count = 0usize;
        for m in 0..self.hail.len()-1 {
            for n in m+1..self.hail.len() {
                if let Some((td,tn)) = self.hail[m].cross_xy(&self.hail[n]) {
                    let t = td as f64 / tn as f64;
                    let pos = self.hail[m].predict(t);
                    if xmin as f64 <= pos.0 && pos.0 <= xmax as f64 &&
                       xmin as f64 <= pos.1 && pos.1 <= xmax as f64 {
                        count += 1;
                    }
                }
            }
        }
        return count;
    }

    // Given velocity, solve for initial position to intercept every hailstone.
    fn guess(&self, vel: Xyz<i64>) -> Option<Hail> {
        // Back-propagate to find valid starting locations for the first two.
        let h0 = self.hail[0].intercept(&vel);
        let h1 = self.hail[1].intercept(&vel);
        // Do those two trajectories overlap at a specific point?
        if let Some((tn,td)) = h0.cross_xy(&h1) {
            let rock = Hail { pos:h0.predict(tn/td), vel:vel };
            if self.hail.iter().all(|h| rock.intersect(h).is_some()) {
                return Some(rock);
            }
        }
        return None;
    }

    // Brute-force search all possible velocities to find initial position.
    fn part2(&self) -> Option<Hail> {
        let vmax = self.hail.iter().map(|h| h.magvel()).max().unwrap();
        for vx in -vmax..=vmax {
            for vy in -vmax..=vmax {
                for vz in -vmax..=vmax {
                    if let Some(h) = self.guess(Xyz(vx,vy,vz)) {
                        return Some(h);
                    }
                }
            }
        }
        return None;
    }
}

fn part1(input: &str) -> usize {
    HailStorm::new(input).part1(200000000000000, 400000000000000)
}

fn part2(input: &str) -> i64 {
    let rock = HailStorm::new(input).part2().unwrap();
    return rock.pos.0 + rock.pos.1 + rock.pos.2;
}

const EXAMPLE: &'static str = "\
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    20, 25, 34 @ -2, -2, -4
    12, 31, 28 @ -1, -2, -1
    20, 19, 15 @  1, -5, -3";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2023, 24).unwrap();

    // Unit tests on provided examples
    assert_eq!(HailStorm::new(EXAMPLE).part1(7, 27), 2);
    assert_eq!(part2(EXAMPLE), 47);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
