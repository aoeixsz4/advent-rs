use std::io;
use crate::input;

fn part1(data: &[i64]) -> usize {
    data.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(data: &[i64]) -> usize {
    data.windows(4).filter(|w| w[3] > w[0]).count()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}