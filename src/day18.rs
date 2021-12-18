use std::fmt::{self, Display, Formatter};
use std::str::Chars;

fn from_str(s: &str) -> Vec<(i64, i64)> {
    let mut depth = 0;
    let mut out: Vec<(i64, i64)> = Vec::new();
    for c in s.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ',' => (),
            d if d.is_ascii_digit() => {
                let n = d.to_digit(10).unwrap() as i64;
                out.push((n, depth));
            }
            _ => panic!("bad input"),
        }
    }
    out
}

fn to_string(vec: &Vec<(i64, i64)>) -> String {
    let mut s = String::new();
    let mut depth = 0;
    let mut left = true;
    let mut last_depth = Vec::new();
    for node in vec {
        if node.1 > depth {
            last_depth.push(depth);
            left = true;
            while node.1 > depth {
                s.push('[');
                depth += 1;
            }
        } else if node.1 < depth {
            left = false;
            while node.1 < depth {
                s.push(']');
                depth -= 1;
            }
        } else if left {
            s.push(',');
            left = false;
        } else if let Some(foo) = last_depth.pop() {
            if foo == node.1 {
                s.push(',');
            }
        }
        s.push_str(&node.0.to_string());
    }
    s
}

fn reduce(vec: &mut Vec<(i64, i64)>) {
    loop {
        if reduce_explode(vec) {
            continue;
        }
        if !reduce_split(vec) {
            break;
        }
    }
}

fn reduce_split(vec: &mut Vec<(i64, i64)>) -> bool {
    for i in 0..vec.len() {
        if vec[i].0 >= 10 {
            let (l, r) = split(&vec[i].0);
            let depth = vec[i].1;
            vec.insert(i + 1, (r, depth + 1));
            vec[i].0 = l;
            vec[i].1 = depth + 1;
            return true;
        }
    }
    false
}

fn split(n: &i64) -> (i64, i64) {
    match n % 2 {
        0 => (n / 2, n / 2),
        1 => ((n - 1) / 2, (n + 1) / 2),
        _ => unreachable!(),
    }
}

fn reduce_explode(vec: &mut Vec<(i64, i64)>) -> bool {
    for i in 0..vec.len() {
        if vec[i].1 >= 4 {
            let (l, r) = explode(vec, i);
            if i > 0 {
                vec[i].0 = l;
            }
            if i < vec.len() - 1 {
                vec[i + 1].0 = r;
            }
            return true;
        }
    }
    false
}

fn explode(vec: &mut Vec<(i64, i64)>, i: usize) -> (i64, i64) {
    if i + 1 >= vec.len() {
        panic!("tried to explode out of bounds");
    }
    let (l, depth) = vec.remove(i);
    if vec[i].1 != depth {
        panic!("exploded non-number pair");
    }
    vec[i].1 = depth - 1;
    let r = vec[i].1;
    vec[i].0 = 0;
    (l, r)
}

pub fn solve() {}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    const EX2: &str = include_str!("day18-ex2.txt");

    #[test]
    fn test() {
        let mut foo = from_str(EX1);
        let mut bar = from_str("[1,2]");
        //println!("foo: {}, depth: idk", &to_string(&foo));
        //println!("foo: {:?}, depth: idk", foo);
        assert_eq!(EX1, &to_string(&foo));
        foo.append(&mut bar);
        println!("after adding [1,2]: {}", &to_string(&foo));

        let mut sample = "[1,1]\n[2,2]\n[3,3]\n[4,4]"
            .lines()
            .map(from_str)
            .collect::<Vec<Vec<(i64, i64)>>>();
        //let mut tot = sample.remove(0);
        //for num in sample.iter_mut() {
        //    tot = tot.append(num);
        //}
        //assert_eq!(&tot.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        //tot = tot.add(Item::from_str("[5,5]"));
        //assert_eq!(&tot.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        //tot = tot.add(Item::from_str("[6,6]"));
        //assert_eq!(&tot.to_string(), "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn test_add_reduce() {
        let mut numbers: Vec<Vec<(i64, i64)>> = EX2.lines().map(from_str).collect();
        let mut tot = numbers.remove(0);
        for num in numbers.iter_mut() {
            tot.append(num);
        }
        println!("result: {}", &to_string(&tot));
    }
}
