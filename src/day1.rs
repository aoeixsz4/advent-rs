use std::io::{self, Write};
use crate::input;

fn part1(data: &[u32], target: u32) -> Option<u32> {
    let (mut i, mut k) = (0, data.len() - 1);
    while i < k {
        let test = data[i] + data[k];
        if test > target {
            k -= 1;
        } else if test < target {
            i += 1;
        } else {
            return Some(data[i] * data[k]);
        }
    }
    None
}

fn part2(data: &[u32]) -> Option<u32> {
    for i in 0 .. data.len() {
        match part1(data, 2020 - data[i]) {
            Some(part_solution) => return Some(data[i] * part_solution),
            None => ()
        }
    }
    None
}

pub fn solve() -> Result<(), io::Error> {
    let mut data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    data.sort(); // both part 1 and part 2 rely on a sorted array
    match part1(data.as_slice(), 2020) {
        Some(solution) => println!("part1: {}", solution.to_string()),
        None => writeln!(io::stderr(), "attempt to solve day 1, part 1 failed")?
    }
    match part2(data.as_slice()) {
        Some(solution) => println!("part2: {}", solution.to_string()),
        None => writeln!(io::stderr(), "attempt to solve day 1, part 2 failed")?
    }
    Ok(())
}