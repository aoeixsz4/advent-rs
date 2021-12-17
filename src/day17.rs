use std::time::Instant;
use std::collections::HashSet;

const MIN_X: i64   = 124;
const MAX_X: i64   = 174;
const MIN_Y: i64  = -123;
const MAX_Y: i64   = -86;

fn tri(n: i64) -> i64 {
    n*(n+1)/2
}

fn x_pos_at_step(vx: i64, n: i64) -> i64 {
    if n < vx {
        tri(vx) - tri(vx-n)
    } else {
        tri(vx)
    }
}

fn y_pos_at_step(vy: i64, n: i64) -> i64 {
    if vy > 0 && n < vy {
        tri(vy) - tri(vy-n)
    } else if vy > 0 {
        tri(vy) - tri(n-vy)
    } else {
        tri(vy) - tri(vy-n)
    }
}

fn is_hit(pos: (i64, i64)) -> bool {
    pos.0 >= MIN_X && pos.0 <= MAX_X
    && pos.1 >= MIN_Y && pos.1 <= MAX_Y
}

fn is_lost(pos: (i64, i64), v: (i64, i64)) -> bool {
    pos.0 > MAX_X || pos.1 < MIN_Y
        || (pos.0 < MIN_X && v.0 == 0)
}

fn try_probe(mut v: (i64, i64)) -> bool {
    let mut pos = (0, 0);
    loop {
        pos.0 += v.0; pos.1 += v.1;
        //println!("({}, {})", pos.0, pos.1);
        if is_hit(pos) { return true; }
        else if is_lost(pos, v) { return false; }
        if v.0 > 0 { v.0 -= 1 };
        if v.0 < 0 { v.1 += 1 };
        v.1 -= 1;
    }
}

fn part1() -> i64 {
    let mut goal_reached = -i64::MAX;
    let x_velocities: Vec<(i64, i64)> = (0 .. 300)
        .map(move |vx| {
            (0..1000).filter_map(move |n| {
            if x_pos_at_step(vx, n) >= MIN_X
                && x_pos_at_step(vx, n) <= MAX_X {
                Some((vx, n))
            } else {
                None
            }
        })
    }).flatten().collect();
    let y_velocities: Vec<(i64, i64)> = (0 .. 300)
        .map(move |vy| {
            (0..1000).filter_map(move |n| {
            if y_pos_at_step(vy, n) >= MIN_Y
                && y_pos_at_step(vy, n) <= MAX_Y {
                Some((vy, n))
            } else {
                None
            }
        })
    }).flatten().collect();
    for (vx, i) in &x_velocities {
        for (vy, j) in &y_velocities {
            if i == j {
                let result = try_probe((*vx, *vy));
                if result && *vy > goal_reached {
                    goal_reached = *vy;
                }
            }
        }
    }
    tri(goal_reached)
}

fn part2() -> usize {
    let x_velocities: Vec<(i64, i64)> = (0 .. 500)
        .map(move |vx| {
            (0..1000).filter_map(move |n| {
            if x_pos_at_step(vx, n) >= MIN_X
                && x_pos_at_step(vx, n) <= MAX_X {
                Some((vx, n))
            } else {
                None
            }
        })
    }).flatten().collect();
    let y_velocities: Vec<(i64, i64)> = (-500 .. 500)
        .map(move |vy| {
            (0..1000).filter_map(move |n| {
            if y_pos_at_step(vy, n) >= MIN_Y
                && y_pos_at_step(vy, n) <= MAX_Y {
                Some((vy, n))
            } else {
                None
            }
        })
    }).flatten().collect();
    let init_velocities: HashSet<(i64, i64)> = x_velocities.iter().flat_map(|(vx, i)| {
        y_velocities.iter().filter_map(move |(vy, j)|{
            if i == j && try_probe((*vx, *vy)) {
                Some((*vx, *vy))
            } else {
                None
            }
        })
    }).collect();
    init_velocities.len()
}

pub fn solve() {
    let t0 = Instant::now();
    let pt1 = part1();
    println!("part2: {}", pt1);
    let t1 = t0.elapsed();
    println!("part1 time: {:?}", t1);
    let t0 = Instant::now();
    let pt2 = part2();
    println!("part2: {}", pt2);
    let t1 = t0.elapsed();
    println!("part2 time: {:?}", t1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_x_range() {
        let vy    = 9;
        let steps = 10;
        let pos = super::y_pos_at_step(vy, steps);
        println!("y pos at step {}, startinge velocity vy {} = {}", steps, vy, pos);
    }

    #[test]
    fn test() {
        println!("{}", x_pos_at_step(3, 0));
        println!("{}", x_pos_at_step(3, 2));
        println!("{}", x_pos_at_step(3, 3));
        println!("{}", x_pos_at_step(3, 4));
        println!("{}", x_pos_at_step(3, 5));
        println!("{}", x_pos_at_step(3, 6));
        println!("{}", x_pos_at_step(3, 7));
        println!("{}", x_pos_at_step(3, 8));
        println!("{}", x_pos_at_step(3, 9));
    }
}