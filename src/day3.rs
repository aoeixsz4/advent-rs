use std::io;
use crate::input;
use bitvec::prelude::*;

const NR_BITS: usize = 12;
const BITMASK: u16 = 0xfff;

fn parse_bitstring(s: &str) -> BitVec<Msb0, u16> {
    s.chars().map_while(|c| {
        if c == '1' { Some(true) } else if c == '0' { Some(false) } else { None }
    }).collect()
}

fn get_bitfields(data: &[String]) -> Vec<BitVec<Msb0, u16>> {
    data.iter().map(|s| parse_bitstring(s)).collect()
}

fn get_bit_counts(data: Vec<BitVec<Msb0, u16>>) -> Vec<usize> {
    (0 .. NR_BITS).map(|i| {
        data.iter().filter(|bit_vec|bit_vec[i]).count()
    }).collect()
}

fn part1(data: &[String]) -> i64 {
    let bit_counts = get_bit_counts(get_bitfields(data));
    println!("bit counts: {:#?}", bit_counts);
    let gamma = bit_counts.iter()
        .map(|n| *n > data.len()/2)
        .collect::<BitVec<Msb0, u16>>()
        .as_raw_slice()[0] >> (16 - NR_BITS);
    println!("gamma: {:#x}", gamma);
    let epsilon = gamma ^ BITMASK;
    gamma as i64 * epsilon as i64
}

fn part2(data: &[String]) -> i64 {
    let bitfields = get_bitfields(data);
    let bit_counts = get_bit_counts(data.iter().map(|s| parse_bitstring(s)).collect());
    let mut bitfields_subset = bitfields.clone();
    for i in 0 .. NR_BITS {
        if bitfields_subset.len() == 1 { break; }
        bitfields_subset = bitfields_subset.into_iter().filter(|bit_vector|
            bit_counts[i] >= (bit_counts.len() / 2) && bit_vector[i]
        ).collect::<Vec<BitVec<Msb0, u16>>>();
    }
    let mut oxygen_generator_rating = 0;
    if bitfields_subset.len() > 0 {
        oxygen_generator_rating = (bitfields_subset[0].as_raw_slice()[0] >> (16 - NR_BITS)) as i64;
    }
    bitfields_subset = bitfields.clone();
    for i in 0 .. NR_BITS {
        if bitfields_subset.len() == 1 { break; }
        bitfields_subset = bitfields_subset.into_iter().filter(|bit_vector|
            bit_counts[i] <= (bit_counts.len() / 2) && !bit_vector[i]
        ).collect::<Vec<BitVec<Msb0, u16>>>();
    }
    let mut co2_scrubber_rating = 0;
    if bitfields_subset.len() > 0 {
        co2_scrubber_rating = (bitfields_subset[0].as_raw_slice()[0] >> (16 - NR_BITS)) as i64;
    }
    oxygen_generator_rating * co2_scrubber_rating
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day3")
        .expect("couldn't open input file for day3 (should be inputs/day3)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}