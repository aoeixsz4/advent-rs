use std::io;
use crate::input;

fn parse_input(data: &[String]) -> Vec<Vec<i64>> {
    data.iter().map(|line| {
        line.split("").filter_map(|d|d.parse::<i64>().ok()).collect::<Vec<i64>>()
    }).collect::<Vec<Vec<i64>>>()
}

fn get_dimensions(map: &Vec<Vec<i64>>) -> (usize, usize) {
    let size_y = map.len();
    if size_y == 0 { panic!("got an empty map!"); }
    let size_x = map[0].len();
    for row in map {
        if row.len() != size_x { panic!("got a mismatching row length!"); }
    }
    (size_y, size_x)
}

fn part1(data: &[String]) -> i64 {
    let mut risk_sum = 0;
    let map = parse_input(data);
    let (size_y, size_x) = get_dimensions(&map);
    for j in 0 .. size_y {
        for i in 0 .. size_x {
            let here = map[j][i];
            if i > 0 && map[j][i-1] <= here { continue; }
            if i < size_x - 1 && map[j][i+1] <= here { continue; }
            if j > 0 && map[j-1][i] <= here { continue; }
            if j < size_y - 1 && map[j+1][i] <= here { continue; }
            risk_sum += here + 1;
        }
    }
    risk_sum
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day9")
        .expect("couldn't open input file for day9 (should be inputs/day9)");
    println!("part1: {}", part1(&data));
    Ok(())
}