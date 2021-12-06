use std::io;
use num::BigInt;
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

fn part2(data: &[String]) -> BigUint {
    let fishies_init = data[0].split(',').map(|nr_string|nr_string.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let zero_vector = vec![0; 1];
    let unit_vector = vec![1; 1];
    let unit = BigUint::new(unit_vector);
    let mut fishy_grid = (0 .. 9).map(|_x| BigUint::new(zero_vector.clone())).collect::<Vec<BigUint>>();
    for fish in fishies_init {
        fishy_grid[fish] += unit.clone();
    }
    for _day in 0 .. 256 {
        let new_spawns = fishy_grid[0].clone();
        for i in 1 .. 9 {
            fishy_grid[i-1] = fishy_grid[i].clone();
        }
        fishy_grid[6] += new_spawns.clone();
        fishy_grid[8] = new_spawns.clone();
    }
    fishy_grid.iter().sum()
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day6")
        .expect("couldn't open input file for day6 (should be inputs/day6)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}