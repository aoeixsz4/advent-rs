const INPUT: &str = include_str!("day1.txt");

fn part1(data: &[i64]) -> usize {
    data.windows(2).filter(|x| x[1] > x[0]).count()
}

fn part2(data: &[i64]) -> usize {
    data.windows(4).filter(|w| w[3] > w[0]).count()
}

pub fn solve() {
    let data: Vec<i64> = INPUT.lines().map(|l|l.parse().unwrap()).collect();
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}