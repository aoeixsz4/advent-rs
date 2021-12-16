use std::collections::HashMap;
use std::iter::FromIterator;
use std::time::Instant;
use std::usize::MAX;
const INPUT: &str = include_str!("day15.txt");
const SIZE: usize = 100;


fn collect_into_array<T, const N: usize>(iter: impl IntoIterator<Item = T>) -> [T; N] {
    let mut iter = iter.into_iter();
    [(); N].map(|_| iter.next().unwrap())
}

/*const DIRS_ALL: [[i32; 2]; 8] = [
    [-1, -1],
    [ 0, -1],
    [-1, -1],
    [-1,  0],
    [ 1,  0],
    [-1,  1],
    [ 0,  1],
    [ 1,  1]
];*/

const DIRS_CARDINAL: [[i32; 2]; 4] = [
    [ 0, -1],
    [-1,  0],
    [ 1,  0],
    [ 0,  1]
];

fn search<const N: usize> (
    node: ((usize, usize), usize),
    visited: &mut HashMap<(usize, usize), usize>,
    unvisited: &mut HashMap<(usize, usize), usize>,
    grid: &[[u8; N]; N]
) -> Vec<(usize, usize)> {
    let (x, y, node_risk_sum) = (node.0.0 as i32, node.0.1 as i32, node.1);
    let unvisited_adjacents: Vec<(usize, usize)> = DIRS_CARDINAL.iter()
        .map(|dir| [x + dir[0], y + dir[1]])
        .filter(|adj|
            adj[0] >= 0 && adj[1] >= 0
            && (adj[0] as usize) < N
            && (adj[1] as usize) < N
        ).map(|adj|(adj[0] as usize, adj[1] as usize))
        .filter(|pos| unvisited.contains_key(pos))
        .collect();
    if unvisited_adjacents.len() == 0 {
        visited.remove(&(x as usize, y as usize));
        return unvisited_adjacents;
    }/* else {
        println!("adjacents: {:?}", unvisited_adjacents);
    }*/
    for pos in unvisited_adjacents.clone() {
        let risk_at = grid[pos.1][pos.0] as usize;
        let risk_guess = unvisited.get_mut(&pos).unwrap();
        if *risk_guess > node_risk_sum + risk_at {
            *risk_guess = node_risk_sum + risk_at
        }
    }
    return unvisited_adjacents;
}

fn get_neighbours<const N: usize> (
    visited: &mut HashMap<(usize, usize), usize>,
    unvisited: &mut HashMap<(usize, usize), usize>,
    grid: &[[u8; N]; N]
) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    for node in visited.clone() {
        neighbours.append(&mut search(node, visited, unvisited, grid));
    }
    neighbours
}

fn get_minimum_risk_path<const N: usize>(
    grid: [[u8; N]; N]
) -> usize {
    let mut unvisited: HashMap<(usize, usize), usize> = HashMap::from_iter((0..N).flat_map(move |y|
        (0..N).map(move |x|((x, y), MAX)
    )));
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    visited.insert((0, 0), 0);
    let mut pos = (0, 0);
    while pos != (N-1, N-1) {
        unvisited.remove(&pos);
        let mut nearest_unvisited = get_neighbours(&mut visited, &mut unvisited, &grid);
        if nearest_unvisited.is_empty() {
            println!("grid: {:?}", grid);
            println!("pos: {:?}", pos);
            println!("visited: {:?}", visited);
            println!("unvisited: {:?}", unvisited);
            panic!("hit a dead end :-(");
        }
        nearest_unvisited.sort_by_key(|k|unvisited.get(k).unwrap());
        pos = nearest_unvisited[0];
        visited.insert(pos, *unvisited.get(&pos).unwrap());
    }
    *visited.get(&pos).unwrap()
}

pub fn solve() {
    const TIMES: u32 = 10;
    let t0 = Instant::now();
    for _ in 0 .. TIMES {
        const EXAMPLE1: &str = include_str!("day15-ex1.txt");
        const EX_SIZE: usize = 10;
        let grid: [[u8; EX_SIZE]; EX_SIZE] = collect_into_array(
            EXAMPLE1.lines().map(|l|
                collect_into_array(l.bytes()
                    .filter(u8::is_ascii_digit)
                    .map(|x|x - b'0'))
            )
        );
        let risk_rating = get_minimum_risk_path(grid);
        assert_eq!(risk_rating, 40);
    }
    let t1 = t0.elapsed();
    eprintln!("test set: {:?}", t1 / TIMES);

    let t0 = Instant::now();
    let grid: [[u8; SIZE]; SIZE] = collect_into_array(
        INPUT.lines().map(|l|
            collect_into_array(l.bytes()
                .filter(u8::is_ascii_digit)
                .map(|x|x - b'0'))
        )
    );
    let risk_rating = get_minimum_risk_path(grid);
    println!("part1: {}", risk_rating);


    let t1 = t0.elapsed();
    eprintln!("part1 set: {:?}", t1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const TIMES: u32 = 10;
        let t0 = Instant::now();
        for _ in 0 .. TIMES {
            const EXAMPLE1: &str = include_str!("day15-ex1.txt");
            const EX_SIZE: usize = 10;
            let grid: [[u8; EX_SIZE]; EX_SIZE] = collect_into_array(
                EXAMPLE1.lines().map(|l|
                    collect_into_array(l.bytes()
                        .filter(u8::is_ascii_digit)
                        .map(|x|x - b'0'))
                )
            );
            let risk_rating = get_minimum_risk_path(grid);
            assert_eq!(risk_rating, 40);
        }
        let t1 = t0.elapsed();
        eprintln!("{:?}", t1 / TIMES);
    }
}