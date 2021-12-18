use std::str::Chars;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
enum Item {
    Number(i64),
    Nested(Box<Item>, Box<Item>)
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Item {
    pub fn from_str(s: &str) -> Item {
        let mut iter = s.strip_prefix("[").unwrap()
            .strip_suffix("]").unwrap().chars();
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
            },
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
                },
                Some(',') => {
                    //println!("got comma, looking for right elements");
                    seen_comma = true;
                },
                Some(']') => {
                    //println!("got ], leaving nested Item");
                    break;
                },
                Some(c) if c.is_ascii_digit() => {
                    //println!("got {}, inserting digit", c);
                    let n = c.to_digit(10).unwrap().into();
                    if seen_comma {
                        right = Item::Number(n);
                    } else {
                        left = Item::Number(n);
                    }
                },
                Some(c) => panic!("invalid input: {}", c),
                None    => break
            }
        }
        if let Item::Number(n) = left { if n == 99 { panic!("bad input, n uninitialised"); }}
        if let Item::Number(n) = right { if n == 99 { panic!("bad input, n uninitialised"); }}
        Item::Nested(Box::new(left), Box::new(right))
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
        let foo = Item::from_str(EX1);
        println!("foo: {}", foo);
        assert_eq!(EX1, &foo.to_string());
    }
}