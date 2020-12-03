use crate::input;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let input_lines = input::get_lines_input("day3")
        .expect("couldn't open input file inputs/day3");
    let mut tree_count = 0;
    let mut absolute_horizontal_index = 0;
    for line in input_lines {
        let index = absolute_horizontal_index % line.len();
        if line.chars().nth(index).unwrap() == '#' {
            tree_count += 1;
        }
        absolute_horizontal_index += 3;
    }
    println!("part1: {}", tree_count);
    Ok(())
}