use std::fmt::{self, Display, Formatter};
use std::str::Chars;

const INPUT: &str = include_str!("day18.txt");

#[derive(Clone, Debug)]
enum Item {
    Number(i64),
    Nested(Box<Item>, Box<Item>),
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Item {
    pub fn from_str(s: &str) -> Item {
        let mut iter = s
            .strip_prefix('[')
            .unwrap()
            .strip_suffix(']')
            .unwrap()
            .chars();
        Item::from_iter(&mut iter)
    }

    pub fn to_string(&self) -> String {
        match &self {
            Item::Number(n) => n.to_string(),
            Item::Nested(l, r) => {
                let mut s = String::from("[");
                s.push_str(&l.to_string());
                s.push(',');
                s.push_str(&r.to_string());
                s.push(']');
                s
            }
        }
    }

    pub fn from_iter(i: &mut Chars) -> Item {
        let mut seen_comma = false;
        let (mut left, mut right) = (Item::Number(99), Item::Number(99));
        loop {
            match i.next() {
                Some('[') => {
                    //println!("got [, entering nested Item");
                    if seen_comma {
                        right = Item::from_iter(i);
                    } else {
                        left = Item::from_iter(i);
                    }
                }
                Some(',') => {
                    //println!("got comma, looking for right elements");
                    seen_comma = true;
                }
                Some(']') => {
                    //println!("got ], leaving nested Item");
                    break;
                }
                Some(c) if c.is_ascii_digit() => {
                    //println!("got {}, inserting digit", c);
                    let n = c.to_digit(10).unwrap().into();
                    if seen_comma {
                        right = Item::Number(n);
                    } else {
                        left = Item::Number(n);
                    }
                }
                Some(c) => panic!("invalid input: {}", c),
                None => break,
            }
        }
        if let Item::Number(n) = left {
            if n == 99 {
                panic!("bad input, n uninitialised");
            }
        }
        if let Item::Number(n) = right {
            if n == 99 {
                panic!("bad input, n uninitialised");
            }
        }
        Item::Nested(Box::new(left), Box::new(right))
    }

    pub fn add(self, other: Item) -> Item {
        let mut outer = Item::Nested(Box::new(self), Box::new(other));
        outer.reduce();
        outer
    }

    fn to_num(&self) -> i64 {
        match self {
            Item::Number(n) => *n,
            Item::Nested(_, _) => panic!("called to_num on nested"),
        }
    }

    pub fn reduce(&mut self) {
        loop {
            if self.reduce_explode() {
                continue;
            }
            if !self.reduce_split() {
                break;
            }
        }
    }

    pub fn magnitude(&self) -> i64 {
        match self {
            Item::Number(n) => *n,
            Item::Nested(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
        }
    }

    pub fn depth(&self) -> usize {
        self._depth(0)
    }

    fn _depth(&self, prev: usize) -> usize {
        match self {
            Item::Number(_) => prev + 1,
            Item::Nested(l, r) => l._depth(prev + 1).max(r._depth(prev + 1)),
        }
    }

    fn reduce_split(&mut self) -> bool {
        match self {
            Item::Number(n) => {
                if *n >= 10 {
                    *self = Item::split(n);
                    true
                } else {
                    false
                }
            }
            Item::Nested(l, r) => {
                if l.reduce_split() {
                    true
                } else {
                    r.reduce_split()
                }
            }
        }
    }

    fn split(n: &i64) -> Item {
        let (left, right) = match n % 2 {
            0 => (n / 2, n / 2),
            1 => ((n - 1) / 2, (n + 1) / 2),
            _ => unreachable!(),
        };
        Item::Nested(Box::new(Item::Number(left)), Box::new(Item::Number(right)))
    }

    fn reduce_explode(&mut self) -> bool {
        let (res, _, _) = self._reduce_explode(0);
        res
    }

    fn _reduce_explode(&mut self, depth: usize) -> (bool, Option<i64>, Option<i64>) {
        match self {
            Item::Number(_) => return (false, None, None),
            Item::Nested(l, r) => {
                if depth >= 4 {
                    //println!("depth {} at nested pair {}, try explode", depth, &self.to_string());
                    let (exp_l, exp_r) = self.explode();
                    return (true, Some(exp_l), Some(exp_r));
                }
                let (l_res, l_exp_l, l_exp_r) = l._reduce_explode(depth + 1);
                if l_res {
                    if let Some(carry) = l_exp_r {
                        r.add_left(carry);
                    }
                    return (true, l_exp_l, None);
                }
                let (r_res, r_exp_l, r_exp_r) = r._reduce_explode(depth + 1);
                if r_res {
                    if let Some(carry) = r_exp_l {
                        l.add_right(carry);
                    }
                    return (true, None, r_exp_r);
                }
            }
        }
        (false, None, None)
    }

    fn explode(&mut self) -> (i64, i64) {
        let res = match self {
            Item::Number(_) => panic!("can't explode a number"),
            Item::Nested(l, r) => (l.to_num(), r.to_num()),
        };
        *self = Item::Number(0);
        res
    }

    fn add_left(&mut self, x: i64) {
        match self {
            Item::Number(n) => *n += x,
            Item::Nested(l, _) => l.add_left(x),
        }
    }

    fn add_right(&mut self, x: i64) {
        match self {
            Item::Number(n) => *n += x,
            Item::Nested(_, r) => r.add_right(x),
        }
    }
}

fn part1() -> i64 {
    let mut numbers: Vec<Item> = INPUT.lines().map(Item::from_str).collect();
    let mut tot = numbers.remove(0);
    for num in numbers {
        tot = tot.add(num);
    }
    tot.magnitude()
}

fn test_pair(a: &Item, b: &Item) -> i64 {
    let (A, B) = (a.clone(), b.clone());
    let sum = A.add(B);
    let res_a = sum.magnitude();
    let (A, B) = (a.clone(), b.clone());
    let res_b = A.add(B).magnitude();
    res_a.max(res_b)
}

fn part2() -> i64 {
    let mut numbers: Vec<Item> = INPUT.lines().map(Item::from_str).collect();
    let mut max_mag = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                max_mag = test_pair(&numbers[i], &numbers[j]).max(max_mag);
            }
        }
    }
    max_mag
}

pub fn solve() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    const EX2: &str = include_str!("day18-ex2.txt");
    const EX3: &str = include_str!("day18-ex3.txt");
    const EX4: &str = include_str!("day18-ex4.txt");
    const EX3_MAGS: [i64; 6] = [143, 1384, 445, 791, 1137, 3488];

    #[test]
    fn test() {
        let mut foo = Item::from_str(EX1);
        let bar = Item::from_str("[1,2]");
        println!("foo: {}, depth: {}", foo, foo.depth());
        assert_eq!(EX1, &foo.to_string());
        let baz = foo.add(bar);
        println!("after adding [1,2]: {}", baz);

        let mut sample = "[1,1]\n[2,2]\n[3,3]\n[4,4]"
            .lines()
            .map(Item::from_str)
            .collect::<Vec<Item>>();
        let mut tot = sample.remove(0);
        for num in sample {
            tot = tot.add(num);
        }
        assert_eq!(&tot.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        tot = tot.add(Item::from_str("[5,5]"));
        assert_eq!(&tot.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        tot = tot.add(Item::from_str("[6,6]"));
        assert_eq!(&tot.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_add_reduce() {
        let mut numbers: Vec<Item> = EX2.lines().map(Item::from_str).collect();
        let mut tot = numbers.remove(0);
        for num in numbers {
            tot = tot.add(num);
        }
        println!("result: {}", tot);
    }

    #[test]
    fn test_magnitude() {
        let numbers: Vec<Item> = EX3.lines().map(Item::from_str).collect();
        for i in 0..numbers.len() {
            assert_eq!(numbers[i].magnitude(), EX3_MAGS[i]);
        }
    }

    #[test]
    fn test_add_reduce_magnitude() {
        let mut numbers: Vec<Item> = EX4.lines().map(Item::from_str).collect();
        let mut tot = numbers.remove(0);
        for num in numbers {
            tot = tot.add(num);
        }
        assert_eq!(tot.magnitude(), 4140);
    }
}
