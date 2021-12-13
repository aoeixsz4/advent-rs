use std::collections::HashSet;
const INPUT: &str = include_str!("day13.txt");

fn part1(input: &str) -> usize {
    let (dots, mirrors) = input.split_once("\n\n").unwrap();
    let mut dots_set = HashSet::new();
    dots.lines().map(|l| l.split_once(",").unwrap())
        .for_each(|(x, y)|{
            dots_set.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
        });
    let first_fold = mirrors.lines().next().unwrap();
    let (axis, index) = first_fold.split_whitespace().skip(2).next().unwrap().split_once("=").unwrap();
    let m = index.parse::<usize>().unwrap();
    match axis {
        "x" => {
            let dots_copy =  dots_set.clone();
            for (x, y) in dots_copy.into_iter().filter(|(x, _y)| *x > m + 1) {
                dots_set.remove(&(x, y));
                if x-m <= m {
                    dots_set.insert((m - (x-m), y));
                }
            }
        },
        "y" => {
            let dots_copy =  dots_set.clone();
            for (x, y) in dots_copy.into_iter().filter(|(_x, y)| *y > m + 1) {
                dots_set.remove(&(x, y));
                if y-m <= m {
                    dots_set.insert((x, m - (y - m)));
                }
            }
        },
        _   => unreachable!()
    }
    dots_set.len()
}

pub fn solve() {
    println!("part1: {}", part1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const EXAMPLE1: &str = include_str!("day13-ex1.txt");
        assert_eq!(part1(EXAMPLE1), 17);
    }
}