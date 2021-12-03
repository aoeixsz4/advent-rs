use std::io;
use crate::input;
use bitvec::prelude::*;

const NR_BITS: usize = 12;
const BITMASK: u16 = 0xfff;

fn common(data: &[String]) -> [usize; NR_BITS] {
    let mut bit_counts: [usize; NR_BITS] = [0; NR_BITS];
    data.iter().for_each(|bitstring| {
        bitstring.char_indices().take(NR_BITS).filter(|(_, c)| *c == '1')
        .for_each(|(i, _)| {
            bit_counts[i] += 1
        })
    });
    bit_counts
}

fn part1(data: &[String]) -> i64 {
    let bit_counts = common(data);
    let bit_vector: BitVec<Msb0, u16> = bit_counts.map(|n| n > data.len()/2)
        .iter().collect();
    let gamma = bit_vector.as_raw_slice()[0] >> (16 - NR_BITS);
    println!("gamma: {:#x}", gamma);
    let epsilon = gamma ^ BITMASK;
    gamma as i64 * epsilon as i64
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day3")
        .expect("couldn't open input file for day3 (should be inputs/day3)");
    println!("part1: {}", part1(&data));
    Ok(())
}