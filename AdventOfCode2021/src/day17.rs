/// Day 17: https://adventofcode.com/2021/day/17
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

struct Region {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
}

impl Region {
    fn new(xmin:i64, xmax:i64, ymin:i64, ymax:i64) -> Region {
        assert!(xmin < xmax);
        assert!(ymin < ymax);
        Region { xmin:xmin, xmax:xmax, ymin:ymin, ymax:ymax }
    }
}

#[derive(Clone)]
struct Projectile {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Projectile {
    fn new(dx: i64, dy: i64) -> Projectile {
        Projectile { x:0, y:0, dx:dx, dy:dy }
    }

    // Move this projectile forward one timestep.
    fn step(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dx -= self.dx.signum();
        self.dy -= 1;
    }

    // Is this projectile within the designated region?
    fn inside(&self, r: &Region) -> bool {
        (r.xmin <= self.x) && (self.x <= r.xmax) &&
        (r.ymin <= self.y) && (self.y <= r.ymax)
    }

    // Could this projectile ever reach the designated region?
    fn could_hit(&self, r: &Region) -> bool {
        let sum_dx = (self.dx*self.dx + self.dx.abs()) / 2;
        let min_dx = self.x + if self.dx < 0 {-sum_dx} else {0};
        let max_dx = self.x + if self.dx > 0 {sum_dx} else {0};
        (r.xmin <= max_dx) && (min_dx <= r.xmax) && (r.ymin <= self.max_height())
    }

    // Max height on current psuedo-parabolic trajectory.
    fn max_height(&self) -> i64 {
        self.y + if self.dy > 0 {(self.dy*self.dy + self.dy) / 2} else {0}
    }

    // Return max-height if this projectile ever passes through target region.
    fn part1(&self, r: &Region) -> Option<i64> {
        let mut now = self.clone();
        while now.could_hit(r) {
            if now.inside(r) {return Some(self.max_height());}
            now.step();
        }
        None
    }
}

// For a given target region, find the max achievable height.
// Note: Upwards trajectories will always pause at y=0 and then
//  stop at zero and then step down by (dy+1).  This puts a firm
//  upper bound on our search range for initial y-velocity.
fn search_part1(r: &Region) -> i64 {
    assert!(r.xmax > 0);
    assert!(r.ymax < 0);
    let mut maxy = 0;
    for dx in 1..r.xmax+1 {
        for dy in 1..r.ymin.abs() {
            let p = Projectile::new(dx, dy);
            // Quick check if we should even try...
            if p.max_height() < maxy {continue;}
            if let Some(y) = p.part1(r) {maxy = y;}
        }
    }
    maxy
}

pub fn solve() {
    let test = Region::new(20, 30, -10, -5);
    let data = Region::new(57, 116, -198, -148);

    assert_eq!(Projectile::new(7,2).part1(&test), Some(3));
    assert_eq!(Projectile::new(6,3).part1(&test), Some(6));
    assert_eq!(Projectile::new(9,0).part1(&test), Some(0));
    assert_eq!(Projectile::new(17,-4).part1(&test), None);
    assert_eq!(Projectile::new(6,9).part1(&test), Some(45));
    assert_eq!(search_part1(&test), 45);

    println!("Part1: {}", search_part1(&data));
}
