const INPUT: &str = include_str!("day7.txt");

fn part1(data: &[&str]) -> i64 {
    let crab_positions = data[0].split(',').map(|s|s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let max = crab_positions.iter().max().unwrap();
    let mut fuel_costs: Vec<i64> = Vec::new();
    for i in 0 .. *max {
        fuel_costs.push(crab_positions.iter().map(|x|(x-i).abs()).sum());
    }
    *fuel_costs.iter().min().unwrap()
}

fn part2(data: &[&str]) -> i64 {
    let crab_positions = data[0].split(',').map(|s|s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let max = crab_positions.iter().max().unwrap();
    let mut fuel_costs: Vec<i64> = Vec::new();
    for i in 0 .. *max {
        fuel_costs.push(crab_positions.iter().map(|x|{
            let n = (x-i).abs();
            (n * (n+1))/2
        }).sum());
    }
    *fuel_costs.iter().min().unwrap()
}

pub fn solve() {
    let data: Vec<&str> = INPUT.lines().collect();
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}