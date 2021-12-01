use std::io;
use crate::input;

fn part1(data: &[u32]) -> u32 {
    data.windows(2).fold(0, |acc, x| {
        if x[1] > x[0] { acc + 1 } else { acc }
    })
}

fn part2(data: &[u32]) -> u32 {
    let sliding_window_sums: Vec<u32> = data.windows(3).map(|y| y.iter().sum()).collect();
    part1(sliding_window_sums.as_slice())
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}