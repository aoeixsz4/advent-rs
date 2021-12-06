use std::io;
use crate::input;

fn part1(data: &[String]) -> usize {
    let mut fishies = data[0].split(',').map(|nr_string|nr_string.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for _day in 0 .. 80 {
        let len = fishies.len();
        for i in 0 .. len {
            if fishies[i] > 0 {
                fishies[i] -= 1;
            } else {
                fishies.push(8);
                fishies[i] = 6;
            }
        }
    }
    fishies.len()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day6")
        .expect("couldn't open input file for day6 (should be inputs/day6)");
    println!("part1: {}", part1(&data));
    Ok(())
}