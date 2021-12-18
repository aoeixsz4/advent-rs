use std::str::Chars;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum PairItem {
    Number(i64),
    Nested(Box<Pair>),
    Uninitialised
}

#[derive(Debug)]
struct Pair {
    left: PairItem,
    right: PairItem
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Pair {
    pub fn new() -> Pair {
        Pair { left: PairItem::Uninitialised, right: PairItem::Uninitialised }
    }

    pub fn new_nested() -> Box<Pair> {
        Box::new(Pair { left: PairItem::Uninitialised, right: PairItem::Uninitialised })
    }

    pub fn from_str(s: &str) -> Pair {
        let mut new_outer = Pair::new();
        let mut iter = s.strip_prefix("[").unwrap()
            .strip_suffix("]").unwrap().chars();
        new_outer.from_iter(&mut iter);
        new_outer
    }

    pub fn to_string(&self) -> String {
        let mut s = String::from("[");
        match &self.left {
            PairItem::Number(n) => s.push_str(&n.to_string()),
            PairItem::Nested(p) => s.push_str(&p.to_string()),
            PairItem::Uninitialised => s.push('_')
        }
        s.push(',');
        match &self.right {
            PairItem::Number(n) => s.push_str(&n.to_string()),
            PairItem::Nested(p) => s.push_str(&p.to_string()),
            PairItem::Uninitialised => s.push('_')
        }
        s.push(']');
        s
    }

    pub fn from_iter(&mut self, i: &mut Chars) {
        let mut seen_comma = false;
        loop {
            match i.next() {
                Some('[') => {
                    //println!("got [, entering nested pair");
                    let mut new = Pair::new_nested();
                    new.from_iter(i);
                    if seen_comma {
                        self.right = PairItem::Nested(new);
                    } else {
                        self.left = PairItem::Nested(new);
                    }
                },
                Some(',') => {
                    //println!("got comma, looking for right elements");
                    seen_comma = true;
                },
                Some(']') => {
                    //println!("got ], leaving nested pair");
                    break
                },
                Some(c) if c.is_ascii_digit() => {
                    //println!("got {}, inserting digit", c);
                    let n = c.to_digit(10).unwrap().into();
                    if seen_comma {
                        self.right = PairItem::Number(n);
                    } else {
                        self.left = PairItem::Number(n);
                    }
                },
                Some(c) => panic!("invalid input: {}", c),
                None    => break
            }
        }
    }
}

pub fn solve() {
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

    #[test]
    fn test() {
        let foo = Pair::from_str(EX1);
        println!("foo: {}", foo);
        assert_eq!(EX1, &foo.to_string());
    }
}