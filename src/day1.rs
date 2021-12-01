use std::io::{self, Write};
use crate::input;

fn part1(data: &[u32]) -> u32 {
    let (mut i, n, mut count) = (1, data.len(), 0);
    while i < n {
        if data[i] > data[i-1] {
            count += 1;
        }
        i += 1;
    }
    count
}

//fn part2(data: &[u32]) -> Option<u32> {
//    for i in 0 .. data.len() {
//        match part1(data, 2020 - data[i]) {
//            Some(part_solution) => return Some(data[i] * part_solution),
//            None => ()
//        }
//    }
//    None
//}

pub fn solve() -> Result<(), io::Error> {
    let mut data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    println!("part1: {}", part1(data.as_slice()));
    //match part2(data.as_slice()) {
    //    Some(solution) => println!("part2: {}", solution.to_string()),
    //    None => writeln!(io::stderr(), "attempt to solve day 1, part 2 failed")?
    //}
    Ok(())
}