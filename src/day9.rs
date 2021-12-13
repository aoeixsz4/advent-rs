const INPUT: &str = include_str!("day9.txt");

fn parse_input(data: &[&str]) -> Vec<Vec<i64>> {
    data.iter().map(|line| {
        line.split("").filter_map(|d|d.parse::<i64>().ok()).collect::<Vec<i64>>()
    }).collect::<Vec<Vec<i64>>>()
}

fn parse_input_2(data: &[&str]) -> Vec<Vec<(i64, bool)>> {
    data.iter().map(|line| {
        line.split("").filter_map(|d|d.parse::<i64>().ok()).map(|d|(d, true)).collect::<Vec<(i64, bool)>>()
    }).collect::<Vec<Vec<(i64, bool)>>>()
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

fn get_dimensions_2(map: &Vec<Vec<(i64, bool)>>) -> (usize, usize) {
    let size_y = map.len();
    if size_y == 0 { panic!("got an empty map!"); }
    let size_x = map[0].len();
    for row in map {
        if row.len() != size_x { panic!("got a mismatching row length!"); }
    }
    (size_y, size_x)
}

fn part1(data: &[&str]) -> i64 {
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

fn count_basin_size(map: &mut Vec<Vec<(i64, bool)>>, mut y: usize, mut x: usize, size_y: usize, size_x: usize) -> usize {
    let mut size = 0;
    let mut neighbours: Vec<(usize, usize)> = Vec::new();
    loop {
        if y < size_y - 1 && map[y+1][x].1 && map[y+1][x].0 < 9 {
            neighbours.push((y+1, x));
        }
        if y > 0 && map[y-1][x].1 && map[y-1][x].0 < 9 {
            neighbours.push((y-1, x));
        }
        if x > 0 && map[y][x-1].1 && map[y][x-1].0 < 9 {
            neighbours.push((y, x-1));
        }
        if map[y][x].1 {
            size += 1;
            map[y][x].1 = false;
        }
        if x < size_x - 1 && map[y][x+1].0 < 9 {
            x += 1;
        } else {
            if let Some(tuple) = neighbours.pop() {
                y = tuple.0;
                x = tuple.1;
            } else {
                break;
            }
        }
    }
    size
}

fn part2(data: &[&str]) -> usize {
    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut map = parse_input_2(data);
    let (size_y, size_x) = get_dimensions_2(&map);
    for j in 0 .. size_y {
        for i in 0 .. size_x {
            if map[j][i].0 < 9 && map[j][i].1 {
                basin_sizes.push(count_basin_size(&mut map, j, i, size_y, size_x));
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes.iter().take(3).product()
}

pub fn solve() {
    let data: Vec<&str> = INPUT.lines().collect();
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}