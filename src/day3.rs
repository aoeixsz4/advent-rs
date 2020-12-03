use crate::input;
use std::io;

pub fn solve() -> Result<(), io::Error> {
    let input_lines = input::get_lines_input("day3")
        .expect("couldn't open input file inputs/day3");
    let mut tree_count = 0;
    let mut absolute_horizontal_index = 0;
    for j in 0 .. input_lines.len() {
        let index = absolute_horizontal_index % input_lines[j].len();
        if input_lines[j].chars().nth(index).unwrap() == '#' {
            tree_count += 1;
        }
        absolute_horizontal_index += 3;
    }
    println!("part1: {}", tree_count);

    // now for part 2
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut answer: u64 = 1;
    for k in 0 .. slopes.len() {
        let mut j = 0;
        let mut i = 0;
        let mut count = 0;
        while j < input_lines.len() {
            let index = i % input_lines[j].len();
            if input_lines[j].chars().nth(index).unwrap() == '#' {
                count += 1;
            }
            i += slopes[k].0;
            j += slopes[k].1;
        }
        answer *= count;
    }
    println!("part2: {}", answer);
    Ok(())
}