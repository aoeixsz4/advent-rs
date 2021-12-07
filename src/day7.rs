use std::io;
use crate::input;

fn part1(data: &[String]) -> i64 {
    let crab_positions = data[0].split(',').map(|s|s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let max = crab_positions.iter().max().unwrap();
    let mut fuel_costs: Vec<i64> = Vec::new();
    for i in 0 .. *max {
        fuel_costs.push(crab_positions.iter().map(|x|(x-i).abs()).sum());
    }
    *fuel_costs.iter().min().unwrap()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day7")
        .expect("couldn't open input file for day6 (should be inputs/day7)");
    println!("part1: {}", part1(&data));
    Ok(())
}