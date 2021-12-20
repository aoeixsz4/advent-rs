use std::time::Instant;
use std::{collections::HashMap, hash::Hash, iter::FromIterator, sync::Arc};

use itertools::Itertools;
use num::traits::Pow;
const INPUT: &str = include_str!("day19.txt");

fn get_scanners(text: &str) -> Vec<Vec<(i64, i64, i64)>> {
    text.split("\n\n")
        .map(|chunks| {
            chunks
                .lines()
                .skip(1)
                .map(|l| {
                    let vec: Vec<i64> = l
                        .split(',')
                        .map(|n| n.parse::<i64>().unwrap_or(0))
                        .collect();
                    (vec[0], vec[1], vec[2])
                })
                .collect()
        })
        .collect()
}

fn get_distances(origin: (i64, i64, i64), beacs: &Vec<(i64, i64, i64)>) -> Vec<f64> {
    beacs
        .iter()
        .map(|(x, y, z)| {
            ((((x - origin.0).pow(2) + (y - origin.1).pow(2) + (z - origin.2).pow(2)) as f64)
                .sqrt())
        })
        .collect()
}

fn translate(coords: (i64, i64, i64), beacs: &Vec<(i64, i64, i64)>) -> Vec<(i64, i64, i64)> {
    beacs
        .iter()
        .map(|(x, y, z)| (x + coords.0, y + coords.1, z + coords.2))
        .collect()
}

fn beacon_dist(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

fn distance_pairs(beacs: &Vec<(i64, i64, i64)>) -> Vec<(Vec<usize>, i64)> {
    let mut pairs: Vec<(Vec<usize>, i64)> = (0..beacs.len())
        .combinations(2)
        .map(|ij| (ij.clone(), beacon_dist(beacs[ij[0]], beacs[ij[1]])))
        .collect();
    pairs.sort_by_key(|(_, d)| *d);
    pairs
}

fn get_pairs(canon: &[(Vec<usize>, i64)], mine: &[(Vec<usize>, i64)]) -> Vec<(usize, usize)> {
    let mut shared: Vec<(usize, usize)> = Vec::new();
    let mut shared_tmp: HashMap<(usize, usize), usize> = HashMap::new();
    let mut pairs: HashMap<Vec<usize>, (Vec<usize>, i64)> = HashMap::new();
    for (a, dist_a) in mine {
        if let Some((b, _)) = canon.iter().find(|(_, dist)| *dist == *dist_a) {
            pairs.insert(a.clone(), (b.clone(), *dist_a));
        }
    }
    for a in pairs.keys() {
        let mut others = pairs.clone();
        let (b, _) = others.remove(a).unwrap();
        for (c, (d, _)) in others {
            if let (Some(ai), Some(bi)) = (
                a.iter().find(|q| c.iter().any(|r| **q == *r)),
                b.iter().find(|q| d.iter().any(|r| **q == *r)),
            ) {
                *shared_tmp.entry((*ai, *bi)).or_insert(1) += 1;
            }
        }
    }
    for ((a, b), count) in shared_tmp {
        if count >= 4 {
            shared.push((a, b));
        }
    }
    shared
}

fn prod(a: &[[i64; 3]; 3], b: &[[i64; 3]; 3]) -> [[i64; 3]; 3] {
    [
        [
            a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0],
            a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1],
            a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2],
        ],
        [
            a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0],
            a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1],
            a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2],
        ],
        [
            a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0],
            a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1],
            a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2],
        ],
    ]
}

fn gen_rot_matrices() -> Vec<Vec<Vec<i64>>> {
    let a = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
        [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    ];
    let b = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    ];
    let c = [
        [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    ];
    let mut out = Vec::new();
    for k in 0..2 {
        for j in 0..4 {
            for i in 0..3 {
                out.push(
                    prod(&prod(&a[i], &b[j]), &c[k])
                        .iter()
                        .map(|o| o.to_vec())
                        .collect(),
                )
            }
        }
    }
    out
}

fn invert_rot_matrix(m: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    vec![
        vec![m[0][0], m[1][0], m[2][0]],
        vec![m[0][1], m[1][1], m[2][1]],
        vec![m[0][2], m[1][2], m[2][2]],
    ]
}

fn rot_set(beacs: &Vec<(i64, i64, i64)>, m: &Vec<Vec<i64>>) -> Vec<(i64, i64, i64)> {
    beacs
        .iter()
        .map(|(x, y, z)| {
            (
                *x * m[0][0] + *y * m[0][1] + *z * m[0][2],
                *x * m[1][0] + *y * m[1][1] + *z * m[1][2],
                *x * m[2][0] + *y * m[2][1] + *z * m[2][2],
            )
        })
        .collect()
}

fn rot(coords: (i64, i64, i64), m: &Vec<Vec<i64>>) -> (i64, i64, i64) {
    let (x, y, z) = (coords.0, coords.1, coords.2);
    (
        x * m[0][0] + y * m[0][1] + z * m[0][2],
        x * m[1][0] + y * m[1][1] + z * m[1][2],
        x * m[2][0] + y * m[2][1] + z * m[2][2],
    )
}

fn unrot(coords: (i64, i64, i64), m: &Vec<Vec<i64>>) -> (i64, i64, i64) {
    let (x, y, z) = (coords.0, coords.1, coords.2);
    let m = invert_rot_matrix(m.to_vec());
    (
        x * m[0][0] + y * m[0][1] + z * m[0][2],
        x * m[1][0] + y * m[1][1] + z * m[1][2],
        x * m[2][0] + y * m[2][1] + z * m[2][2],
    )
}

fn diff_pairs(
    canon_beacs: &Vec<(i64, i64, i64)>,
    beacs: &Vec<(i64, i64, i64)>,
    pairs: &Vec<(usize, usize)>,
) -> Option<((i64, i64, i64), Vec<(i64, i64, i64)>)> {
    for m in 0..pairs.len() {
        let (ti, tj) = pairs[m];
        let (dx, dy, dz) = (
            canon_beacs[tj].0 - beacs[ti].0,
            canon_beacs[tj].1 - beacs[ti].1,
            canon_beacs[tj].2 - beacs[ti].2,
        );
        let mut hits = 0;
        let mut hitsv = Vec::new();
        for (i, j) in pairs.iter() {
            if beacs[*i].0 + dx == canon_beacs[*j].0
                && beacs[*i].1 + dy == canon_beacs[*j].1
                && beacs[*i].2 + dz == canon_beacs[*j].2
            {
                hits += 1;
                hitsv.push(*i);
            }
        }
        if hits >= 12 {
            //println!("hits: {}", hits);
            //println!("hitsv: {:?}", hitsv);
            return Some((
                (dx, dy, dz),
                beacs
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !hitsv.contains(i))
                    .map(|(_, (a, b, c))| (*a + dx, *b + dy, *c + dz))
                    .collect(),
            ));
        }
    }
    None
}

fn try_rotations(
    canon_beacs: &Vec<(i64, i64, i64)>,
    beacs: &Vec<(i64, i64, i64)>,
    pairs: &Vec<(usize, usize)>,
    rotations: &Vec<Vec<Vec<i64>>>,
) -> Option<((i64, i64, i64), Vec<(i64, i64, i64)>)> {
    for rot_matrix in rotations {
        let beacs_rot = rot_set(beacs, &rot_matrix);
        if let Some(ret) = diff_pairs(canon_beacs, &beacs_rot, pairs) {
            return Some(ret);
        }
    }
    None
}

fn part1(txt: &str) -> usize {
    let mut scanners = get_scanners(txt);
    let mut beacons_canon = scanners.remove(0);
    let mut canon_beac_pairs: Vec<(Vec<usize>, i64)>;
    let rots = gen_rot_matrices();
    while !scanners.is_empty() {
        canon_beac_pairs = distance_pairs(&beacons_canon);
        let mut scans_copy = scanners.clone();
        for (i, scanner) in scans_copy.iter_mut().enumerate() {
            let beac_pairs = distance_pairs(scanner);
            let matching_pairs = get_pairs(&canon_beac_pairs, &beac_pairs);
            if let Some((_coords, mut corrected_beacs)) =
                try_rotations(&beacons_canon, scanner, &matching_pairs, &rots)
            {
                //println!("{} beacs", scanner.len());
                /*println!(
                    "got coords ({}, {}, {}) for scanner {} with 12 matches and {} new beacons",
                    coords.0,
                    coords.1,
                    coords.2,
                    i,
                    rotated_beacs.len()
                );*/
                /*println!(
                    "original beacs: {:?}, corrected beacs other csanner: {:?}",
                    beacons_canon, corrected_beacons
                );*/
                beacons_canon.append(&mut corrected_beacs);
                scanners.remove(i);
                //println!("standard beacons: {}", beacons_canon.len());
                break;
            }
        }
    }
    beacons_canon.len()
}

pub fn solve() {
    let t0 = Instant::now();
    let pt1 = part1(INPUT);
    println!("part1: {}", pt1);
    let t1 = t0.elapsed();
    //println!("part2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX2: &str = include_str!("day19-ex2.txt");

    #[test]
    fn test() {
        const TIMES: u32 = 10;
        let t0 = Instant::now();
        for _ in 0..TIMES {
            assert_eq!(part1(EX2), 79);
        }
        let t1 = t0.elapsed();
        println!("duration: {:?}", t1 / TIMES);
    }

    #[test]
    fn test_gen_m() {
        println!("{:?}", gen_rot_matrices());
        println!("len: {}", gen_rot_matrices().len());
    }

    #[test]
    fn test_rot() {
        let rots = gen_rot_matrices();
        for r in rots.clone() {
            println!("{:?}", r);
        }
        println!("{} orientations", rots.len());
        let mut foo = (140, -50, 100);
        //println!("orig: ({}, {}, {})", foo.0, foo.1, foo.2);
        for (i, rotation) in rots.iter().enumerate() {
            let (x, y, z) = rot(foo, &rotation);
            //println!("rotated: ({}, {}, {})", x, y, z);
            let (x, y, z) = unrot((x, y, z), &rotation);
            assert_eq!(
                (x, y, z),
                (140, -50, 100),
                "unrotated bad index {}, rot matrix: {:?}",
                i,
                rotation
            );
        }
    }
}
