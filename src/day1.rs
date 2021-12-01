use std::io;
use crate::input;

fn part1(data: &[u32]) -> u32 {
    data.windows(2).fold(0, |acc, x| {
        if x[1] > x[0] { acc + 1 } else { acc }
    })
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
    let data = input::get_numeric_input("day1")
        .expect("couldn't open input file for day1 (should be inputs/day1)");
    println!("part1: {}", part1(&data));
    //match part2(data.as_slice()) {
    //    Some(solution) => println!("part2: {}", solution.to_string()),
    //    None => writeln!(io::stderr(), "attempt to solve day 1, part 2 failed")?
    //}
    Ok(())
}