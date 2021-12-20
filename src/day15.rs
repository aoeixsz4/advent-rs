use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::time::Instant;
use std::usize::MAX;
const INPUT: &str = include_str!("day15.txt");
const SIZE: usize = 100;
const BIG_GRID_SIZE: usize = 500;

fn copy_smol_into(dest: &mut [[u8; BIG_GRID_SIZE]; BIG_GRID_SIZE], src: &[[u8; SIZE]; SIZE]) {
    for j in 0..SIZE {
        for i in 0..SIZE {
            dest[j][i] = src[j][i];
        }
    }
}

fn copy_intra(grid: &mut [[u8; BIG_GRID_SIZE]; BIG_GRID_SIZE], k: usize, m: usize) {
    if k == 0 && m == 0 {
        panic!("can't copy internally to grid section 0, 0");
    }
    if m == 0 {
        for j in 0..SIZE {
            for i in 0..SIZE {
                let mut x = grid[m * SIZE + j][(k - 1) * SIZE + i];
                if x == 9 {
                    x = 1
                } else {
                    x += 1
                }
                grid[m * SIZE + j][k * SIZE + i] = x;
            }
        }
    } else {
        for j in 0..SIZE {
            for i in 0..SIZE {
                let mut x = grid[(m - 1) * SIZE + j][k * SIZE + i];
                if x == 9 {
                    x = 1
                } else {
                    x += 1
                }
                grid[m * SIZE + j][k * SIZE + i] = x;
            }
        }
    }
}

fn gen_big_grid(grid: [[u8; SIZE]; SIZE]) -> [[u8; BIG_GRID_SIZE]; BIG_GRID_SIZE] {
    let mut big_grid = [[0u8; BIG_GRID_SIZE]; BIG_GRID_SIZE];
    for m in 0..5 {
        for k in 0..5 {
            if m == 0 && k == 0 {
                copy_smol_into(&mut big_grid, &grid);
            } else {
                copy_intra(&mut big_grid, k, m);
            }
        }
    }
    big_grid
}

fn collect_into_array<T, const N: usize>(iter: impl IntoIterator<Item = T>) -> [T; N] {
    let mut iter = iter.into_iter();
    [(); N].map(|_| iter.next().unwrap())
}

const DIRS_CARDINAL: [[i32; 2]; 4] = [[0, -1], [-1, 0], [1, 0], [0, 1]];

fn search<const N: usize>(
    pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), usize>,
    unvisited: &mut HashMap<(usize, usize), usize>,
    todo: &mut BTreeMap<usize, Vec<(usize, usize)>>,
    grid: &[[u8; N]; N],
) {
    let node_risk_sum = *visited.get(&pos).unwrap();
    let (x, y) = (pos.0 as i32, pos.1 as i32);
    let unvisited_adjacents: Vec<(usize, usize)> = DIRS_CARDINAL
        .iter()
        .map(|dir| [x + dir[0], y + dir[1]])
        .filter(|adj| adj[0] >= 0 && adj[1] >= 0 && (adj[0] as usize) < N && (adj[1] as usize) < N)
        .map(|adj| (adj[0] as usize, adj[1] as usize))
        .filter(|pos| unvisited.contains_key(pos))
        .collect();
    if !unvisited_adjacents.is_empty() {
        visited.remove(&(x as usize, y as usize));
    }
    for pos in unvisited_adjacents {
        let risk_at = grid[pos.1][pos.0] as usize;
        let risk_guess = unvisited.get_mut(&pos).unwrap();
        if *risk_guess > node_risk_sum + risk_at {
            *risk_guess = node_risk_sum + risk_at;
            let queue_risk_entry = todo.entry(*risk_guess).or_insert_with(Vec::new);
            queue_risk_entry.push(pos);
        }
    }
}

fn get_minimum_risk_path<const N: usize>(grid: [[u8; N]; N]) -> usize {
    let mut unvisited: HashMap<(usize, usize), usize> =
        HashMap::from_iter((0..N).flat_map(move |y| (0..N).map(move |x| ((x, y), MAX))));
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut todo: BTreeMap<usize, Vec<(usize, usize)>> = BTreeMap::new();
    let mut pos = (0, 0);
    visited.insert(pos, 0);
    while pos != (N - 1, N - 1) {
        unvisited.remove(&pos);
        search(pos, &mut visited, &mut unvisited, &mut todo, &grid);
        if todo.is_empty() {
            panic!("hit a dead end :-(");
        }
        let key = *todo.keys().next().unwrap();
        let entry = todo.get_mut(&key).unwrap();
        pos = entry.pop().unwrap();
        if entry.is_empty() {
            todo.remove(&key);
        }
        visited.insert(pos, key);
    }
    *visited.get(&pos).unwrap()
}

pub fn solve() {
    const TIMES: u32 = 1000;
    let t0 = Instant::now();
    let mut risk_rating = 0;
    for _ in 0..TIMES {
        let grid: [[u8; SIZE]; SIZE] =
            collect_into_array(INPUT.lines().map(|l| {
                collect_into_array(l.bytes().filter(u8::is_ascii_digit).map(|x| x - b'0'))
            }));
        risk_rating = get_minimum_risk_path(grid);
    }
    println!("part1: {}", risk_rating);
    let t1 = t0.elapsed();
    eprintln!("part1 set: {:?}", t1 / TIMES);

    const TIMES2: u32 = 10;
    let t0 = Instant::now();
    let mut risk_rating = 0;
    for _ in 0..TIMES2 {
        let grid: [[u8; SIZE]; SIZE] =
            collect_into_array(INPUT.lines().map(|l| {
                collect_into_array(l.bytes().filter(u8::is_ascii_digit).map(|x| x - b'0'))
            }));
        let big_grid: [[u8; BIG_GRID_SIZE]; BIG_GRID_SIZE] = gen_big_grid(grid);
        risk_rating = get_minimum_risk_path(big_grid);
    }
    println!("part2: {}", risk_rating);
    let t1 = t0.elapsed();
    println!("part2 set: {:?}", t1 / TIMES2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const TIMES: u32 = 1;
        let t0 = Instant::now();
        for _ in 0..TIMES {
            const EXAMPLE1: &str = include_str!("day15-ex1.txt");
            const EX_SIZE: usize = 10;
            let grid: [[u8; EX_SIZE]; EX_SIZE] = collect_into_array(EXAMPLE1.lines().map(|l| {
                collect_into_array(l.bytes().filter(u8::is_ascii_digit).map(|x| x - b'0'))
            }));
            let risk_rating = get_minimum_risk_path(grid);
            assert_eq!(risk_rating, 40);
            const EXAMPLE2: &str = include_str!("day15-ex2.txt");
            let big_grid: [[u8; 50]; 50] = collect_into_array(EXAMPLE2.lines().map(|l| {
                collect_into_array(l.bytes().filter(u8::is_ascii_digit).map(|x| x - b'0'))
            }));
            let risk_rating = get_minimum_risk_path(big_grid);
            assert_eq!(risk_rating, 315);
        }
        let t1 = t0.elapsed();
        eprintln!("{:?}", t1 / TIMES);
    }
}
