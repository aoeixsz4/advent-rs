use std::io;
use crate::input;

struct Position {
    horizontal: i64,
    depth: i64
}

fn part1(data: &[String]) -> i64 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0
    };
    data.iter().map(|l| l.split_once(' '))
        .for_each(|s| {
            if let Some((direction, amount)) = s {
                match direction {
                    "forward" => pos.horizontal += amount.parse::<i64>().unwrap(),
                    "up" => pos.depth -= amount.parse::<i64>().unwrap(),
                    "down" => pos.depth += amount.parse::<i64>().unwrap(),
                    _ => ()
                }
            }
        });
    pos.horizontal * pos.depth
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day2")
        .expect("couldn't open input file for day2 (should be inputs/day2)");
    println!("part1: {}", part1(&data));
    Ok(())
}