use std::io;
use crate::input;

const SIZE_Y: usize = 10;
const SIZE_X: usize = 10;

fn parse_input(data: &[String]) -> Vec<Vec<i64>> {
    data.iter().map(|line| {
        line.split("").filter_map(|d|d.parse::<i64>().ok()).collect::<Vec<i64>>()
    }).collect::<Vec<Vec<i64>>>()
}

fn increment_all(octopus_grid: &mut Vec<Vec<i64>>) {
    for row in octopus_grid {
        for octopus in row {
            *octopus += 1
        }
    }
}

fn cascade_flashers(octopus_grid: &mut Vec<Vec<i64>>) -> i64 {
    let mut flasher_stack: Vec<(usize, usize)> = Vec::new();
    let mut flasher_count = 0;
    for y in 0 .. 10 {
        for x in 0 .. 10 {
            if octopus_grid[y][x] > 9 {
                flasher_stack.push((y, x));
                flasher_count += 1;
            }
        }
    }
    while let Some((y, x)) = flasher_stack.pop() {
        if y > 0 && x > 0 {
            if octopus_grid[y-1][x-1] == 9 {
                flasher_stack.push((y-1, x-1));
                flasher_count += 1;
            }
            octopus_grid[y-1][x-1] += 1;
        }
        if y > 0 {
            if octopus_grid[y-1][x] == 9 {
                flasher_stack.push((y-1, x));
                flasher_count += 1;
            }
            octopus_grid[y-1][x] += 1;
        }
        if y > 0 && x < SIZE_X - 1 {
            if octopus_grid[y-1][x+1] == 9 {
                flasher_stack.push((y-1, x+1));
                flasher_count += 1;
            }
            octopus_grid[y-1][x+1] += 1;
        }
        if x > 0 {
            if octopus_grid[y][x-1] == 9 {
                flasher_stack.push((y, x-1));
                flasher_count += 1;
            }
            octopus_grid[y][x-1] += 1;
        }
        if x < SIZE_X - 1 {
            if octopus_grid[y][x+1] == 9 {
                flasher_stack.push((y, x+1));
                flasher_count += 1;
            }
            octopus_grid[y][x+1] += 1;
        }
        if y < SIZE_Y - 1 && x > 0 {
            if octopus_grid[y+1][x-1] == 9 {
                flasher_stack.push((y+1, x-1));
                flasher_count += 1;
            }
            octopus_grid[y+1][x-1] += 1;
        }
        if y < SIZE_Y - 1 {
            if octopus_grid[y+1][x] == 9 {
                flasher_stack.push((y+1, x));
                flasher_count += 1;
            }
            octopus_grid[y+1][x] += 1;
        }
        if y < SIZE_Y - 1 && x < SIZE_X - 1 {
            if octopus_grid[y+1][x+1] == 9 {
                flasher_stack.push((y+1, x+1));
                flasher_count += 1;
            }
            octopus_grid[y+1][x+1] += 1;
        }
    }
    flasher_count
}

fn zero_flashed(octopus_grid: &mut Vec<Vec<i64>>) {
    for row in octopus_grid {
        for octopus in row {
            if *octopus > 9 { *octopus = 0; }
        }
    }
}

fn part1(data: &[String]) -> i64 {
    let mut flashed_count = 0;
    let mut octopus_grid = parse_input(data);
    for _i in 0 .. 100 {
        increment_all(&mut octopus_grid);
        flashed_count += cascade_flashers(&mut octopus_grid);
        zero_flashed(&mut octopus_grid);
    }
    flashed_count
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day11")
        .expect("couldn't open input file for day11 (should be inputs/day11)");
    println!("part1: {}", part1(&data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [
            "5483143223".to_string(),
            "2745854711".to_string(),
            "5264556173".to_string(),
            "6141336146".to_string(),
            "6357385478".to_string(),
            "4167524645".to_string(),
            "2176841721".to_string(),
            "6882881134".to_string(),
            "4846848554".to_string(),
            "5283751526".to_string(),
        ];
        assert_eq!(part1(&input), 1656);
    }
}