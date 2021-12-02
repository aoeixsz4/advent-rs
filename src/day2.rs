use std::io;
use crate::input;

struct Position {
    horizontal: u32,
    depth: u32
}

fn part1(data: &[String]) -> u32 {
    let mut pos = Position {
        horizontal: 0,
        depth: 0
    };
    data.iter().map(|l| l.splitn(2, ' '))
        .for_each(|mut s| {
            match s.next().unwrap() {
                "forward" => pos.horizontal += s.next().unwrap().parse::<u32>().unwrap(),
                "up" => pos.depth -= s.next().unwrap().parse::<u32>().unwrap(),
                "down" => pos.depth += s.next().unwrap().parse::<u32>().unwrap(),
                _ => ()
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