use crate::input;
use std::io;

#[derive(Debug)]
struct PassPolicy
{
    params: (usize, usize), // for old policy: min, max, for new policy, potential indices
    glyph: char
}

#[derive(Debug)]
struct PassEntry<'a>
{
    policy: PassPolicy,
    password: &'a str
}

impl<'a> PassEntry<'a>
{
    pub fn new(input: &'a str) -> Self
    {
        let mut split: Vec<&'a str> = input.split(": ").collect();
        assert_eq!(split.len(), 2);
        let policy_str = split[0];
        let password = split[1];
        split = policy_str.split(" ").collect();
        assert_eq!(split.len(), 2);
        assert_eq!(split[1].len(), 1);
        let glyph = split[1].chars().nth(0).unwrap();
        split = split[0].split("-").collect();
        let policy = PassPolicy
        {
            params: (split[0].parse::<usize>().unwrap(),
                        split[1].parse::<usize>().unwrap()),
            glyph
        };
        PassEntry
        {
            policy,
            password
        }
    }

    pub fn is_valid_old(&self) -> bool
    {
        let mut glyph_count = 0;
        for glyph in self.password.chars()
        {
            if glyph == self.policy.glyph
            {
                glyph_count += 1;
            }
        }

        if glyph_count < self.policy.params.0
            || glyph_count > self.policy.params.1
        {
            false
        }
        else
        {
            true
        }
    }

    pub fn is_valid_new(&self) -> bool
    {
        let c1 = self.password.chars().nth(self.policy.params.0 - 1).unwrap_or('\0');
        let c2 = self.password.chars().nth(self.policy.params.1 - 1).unwrap_or('\0');
        let mut hits = 0;
        if c1 == self.policy.glyph { hits += 1; }
        if c2 == self.policy.glyph { hits += 1; }
        if hits == 1 { true }
        else { false }
    }
}

pub fn solve() -> Result<(), io::Error> {
    let input_lines = input::get_lines_input("day2")
        .expect("couldn't open input file inputs/day2");
    let mut count_old: u32 = 0;
    let mut count_new: u32 = 0;
    for line in input_lines {
        let entry = PassEntry::new(&line);
        if entry.is_valid_old() { count_old += 1; }
        if entry.is_valid_new() { count_new += 1; }
    }
    println!("part1: {}", count_old);
    println!("part2: {}", count_new);
    Ok(())
}