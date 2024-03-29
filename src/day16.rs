use bitvec::prelude::*;
use hex::FromHex;
use std::time::Instant;

const INPUT: &str = include_str!("day16.txt");

struct BitReader {
    bitvec: BitVec<Msb0, u8>,
    cursor: usize,
}

impl BitReader {
    pub fn new(bv: BitVec<Msb0, u8>) -> BitReader {
        BitReader {
            bitvec: bv,
            cursor: 0,
        }
    }

    pub fn shift(&mut self) -> bool {
        let ret = self.bitvec[self.cursor];
        self.cursor += 1;
        ret
    }

    pub fn read(&mut self, n: usize) -> u16 {
        if n > 16 {
            panic!("cannot read more than 16 bits at once");
        }
        let mut ret = 0u16;
        let bit_slice = self.read_bits(n);
        for i in 0..n {
            ret |= ((bit_slice[i] as u16) << (n - 1)) - i as u16;
        }
        ret
    }

    pub fn read_bits(&mut self, n: usize) -> &BitSlice<Msb0, u8> {
        let ret = &self.bitvec[self.cursor..self.cursor + n];
        self.cursor += n;
        ret
    }

    pub fn skip(&mut self, n: usize) {
        self.cursor += n;
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

fn consume_literal(r: &mut BitReader) {
    while r.shift() {
        r.skip(4);
    }
    r.skip(4);
}

fn calculate_literal(r: &mut BitReader) -> u64 {
    let mut ret: u64 = 0;
    let mut nibls = Vec::new();
    while r.shift() {
        nibls.push(r.read(4) as u8);
    }
    nibls.push(r.read(4) as u8);
    for i in 0..nibls.len() {
        ret |= (nibls[nibls.len() - (i + 1)] as u64) << (4 * i)
    }
    ret
}

fn parse_packet(r: &mut BitReader) -> u64 {
    let mut version_sum = r.read(3) as u64;
    let type_id = r.read(3);
    if type_id == 4 {
        consume_literal(r);
        return version_sum as u64;
    }
    if r.shift() {
        let nr_packets = r.read(11);
        for _ in 0..nr_packets {
            version_sum += parse_packet(r);
        }
    } else {
        let len = r.read(15);
        let packet_end = len as usize + r.cursor();
        while r.cursor() != packet_end {
            if r.cursor() > packet_end {
                panic!("packet size error");
            }
            version_sum += parse_packet(r);
        }
    }
    version_sum
}

fn calculate_packet(r: &mut BitReader) -> u64 {
    let _version = r.read(3) as u64;
    let type_id = r.read(3);
    let mut values = Vec::new();
    if type_id == 4 {
        return calculate_literal(r);
    }
    if r.shift() {
        let nr_packets = r.read(11);
        for _ in 0..nr_packets {
            values.push(calculate_packet(r));
        }
    } else {
        let len = r.read(15);
        let packet_end = len as usize + r.cursor();
        while r.cursor() != packet_end {
            if r.cursor() > packet_end {
                panic!("packet size error");
            }
            values.push(calculate_packet(r));
        }
    }
    match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => {
            if values[0] > values[1] {
                1
            } else {
                0
            }
        }
        6 => {
            if values[0] < values[1] {
                1
            } else {
                0
            }
        }
        7 => {
            if values[0] == values[1] {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

pub fn solve() {
    let t0 = Instant::now();
    let bytes = Vec::from_hex(INPUT).expect("asdf");
    let bit_vec: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
    let mut reader = BitReader::new(bit_vec);
    let sum = parse_packet(&mut reader);
    let t1 = t0.elapsed();
    println!("part1: {}", sum);
    println!("part1 time: {:?}", t1);

    let t0 = Instant::now();
    let bytes = Vec::from_hex(INPUT).expect("asdf");
    let bit_vec: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
    let mut reader = BitReader::new(bit_vec);
    let sum = calculate_packet(&mut reader);
    let t1 = t0.elapsed();
    println!("part2: {}", sum);
    println!("part2 time: {:?}", t1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_1_BIN: &str = "110100101111111000101000";
    const EX_2_BIN: &str = "00111000000000000110111101000101001010010001001000000000";
    const EX_3_BIN: &str = "11101110000000001101010000001100100000100011000001100000";
    const EX_1: &str = "8A004A801A8002F478";
    const EX_2: &str = "620080001611562C8802118E34";
    const EX_3: &str = "C0015000016115A2E0802F182340";
    const EX_4: &str = "A0016C880162017C3686B18A3D4780";
    const SOLUTION_1: u64 = 16;
    const SOLUTION_2: u64 = 12;
    const SOLUTION_3: u64 = 23;
    const SOLUTION_4: u64 = 31;

    const PART2_EXAMPLES: [(&str, u64); 8] = [
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];

    fn from_ascii_bit(enc: u8) -> u8 {
        match enc {
            b'0' => 0,
            b'1' => 1,
            _ => panic!("not an ASCII bit"),
        }
    }

    fn from_binary_string(s: &str) -> BitVec<Msb0, u8> {
        if s.bytes().any(|b| b != b'0' && b != b'1') {
            panic!("not a binary string");
        }
        let byte_string = s.as_bytes();
        assert_eq!(byte_string.len() % 8, 0);
        BitVec::from_vec(
            byte_string
                .chunks(8)
                .map(|byte| (0..8).map(move |i| from_ascii_bit(byte[i]) << 7 - i).sum())
                .collect::<Vec<u8>>(),
        )
    }

    fn test_binary_string(ex: &str) {
        let bitvec = from_binary_string(ex);
        let mut reader = BitReader::new(bitvec);
        let _sum = parse_packet(&mut reader);
    }

    fn test_calc_binary_string(ex: &str) -> u64 {
        let bitvec = from_binary_string(ex);
        let mut reader = BitReader::new(bitvec);
        calculate_packet(&mut reader)
    }

    #[test]
    fn test_binary_parsing() {
        let _bit_vec = from_binary_string(EX_1_BIN);
    }

    #[test]
    fn test_literal() {
        test_binary_string(EX_1_BIN);
    }

    #[test]
    fn test_calc_literal() {
        let value = test_calc_binary_string(EX_1_BIN);
        assert_eq!(value, 2021);
    }

    #[test]
    fn test_subtype0() {
        test_binary_string(EX_2_BIN);
    }

    #[test]
    fn test_subtype1() {
        test_binary_string(EX_3_BIN);
    }

    #[test]
    fn test_hex_examples() {
        for (test_set, solution) in [
            (EX_1, SOLUTION_1),
            (EX_2, SOLUTION_2),
            (EX_3, SOLUTION_3),
            (EX_4, SOLUTION_4),
        ] {
            let bytes = Vec::from_hex(test_set).expect("asdf");
            let bit_vec: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
            let mut reader = BitReader::new(bit_vec);
            let sum = parse_packet(&mut reader);
            assert_eq!(sum, solution);
        }
    }

    #[test]
    fn test_pt2_examples() {
        for (test_set, solution) in PART2_EXAMPLES {
            let bytes = Vec::from_hex(test_set).expect("asdf");
            let bit_vec: BitVec<Msb0, u8> = BitVec::from_vec(bytes);
            let mut reader = BitReader::new(bit_vec);
            let sum = calculate_packet(&mut reader);
            assert_eq!(sum, solution);
        }
    }
}
