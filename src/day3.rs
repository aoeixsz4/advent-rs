use std::io;
use crate::input;
use bitvec::prelude::*;

const NR_BITS: usize = 12;
const BITMASK: usize = 0xfff;

fn part1(data: &[String]) -> usize {
    let mut bit_counts: [usize; NR_BITS] = [0; NR_BITS];
    let len = data.len();
    data.iter().for_each(|bitstring| {
        bitstring.char_indices().take(NR_BITS).filter(|(_, c)| *c == '1')
        .for_each(|(i, _)| {
            bit_counts[i] += 1
        })
    });
    let bitvec = bit_counts.map(|n| n > len/2)
        .iter().collect::<BitVec<Msb0, usize>>();
    let gamma = bitvec.as_raw_slice()[0] >> (64 - NR_BITS);
    println!("gamma: {:#x}", gamma);
    let epsilon = gamma ^ BITMASK;
    gamma * epsilon
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day3")
        .expect("couldn't open input file for day3 (should be inputs/day3)");
    println!("part1: {}", part1(&data));
    Ok(())
}