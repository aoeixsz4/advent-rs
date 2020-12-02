use std::io::{self, Write};
use crate::input;

fn part1(data: &mut [u32]) -> Option<u32> {
    data.sort();
    let (mut i, mut k) = (0, data.len() - 1);
    let target = 2020;
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

pub fn solve() -> Result<(), io::Error> {
    let mut data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    match part1(data.as_mut_slice()) {
        Some(solution) => println!("{}", solution.to_string()),
        None => writeln!(io::stderr(), "attempt to solve day 1 failed")?
    }
    Ok(())
}