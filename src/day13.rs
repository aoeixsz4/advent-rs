use std::collections::HashSet;
const INPUT: &str = include_str!("day13.txt");
const HEIGHT: usize = 40;
const WIDTH: usize = 200;

fn fold_paper(input: &str, fold_once: bool) -> HashSet<(usize, usize)> {
    let (dots, mirrors) = input.split_once("\n\n").unwrap();
    let mut dots_set = HashSet::new();
    dots.lines()
        .map(|l| l.split_once(",").unwrap())
        .for_each(|(x, y)| {
            dots_set.insert((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
        });
    for (axis, m) in mirrors
        .lines()
        .map(|l| {
            l.split_whitespace()
                .nth(2)
                .unwrap()
                .split_once("=")
                .unwrap()
        })
        .map(|(axis, index)| (axis, index.parse::<usize>().unwrap()))
    {
        match axis {
            "x" => {
                let dots_copy = dots_set.clone();
                for (x, y) in dots_copy.into_iter().filter(|(x, _y)| *x > m) {
                    dots_set.remove(&(x, y));
                    if x - m <= m {
                        dots_set.insert((m - (x - m), y));
                    }
                }
            }
            "y" => {
                let dots_copy = dots_set.clone();
                for (x, y) in dots_copy.into_iter().filter(|(_, y)| *y > m) {
                    dots_set.remove(&(x, y));
                    if y - m <= m {
                        dots_set.insert((x, m - (y - m)));
                    }
                }
            }
            _ => unreachable!(),
        }
        if fold_once {
            break;
        }
    }
    dots_set
}

fn part1(input: &str) -> usize {
    fold_paper(input, true).len()
}

fn part2(input: &str) {
    let mut grid = [[false; WIDTH]; HEIGHT];
    for (x, y) in fold_paper(input, false).into_iter() {
        if x >= WIDTH || y >= HEIGHT {
            panic!("grid too small");
        }
        grid[y][x] = true;
    }
    for line in grid.iter().filter_map(|row| {
        if row.iter().all(|p| !*p) {
            None
        } else {
            Some(
                row.iter()
                    .map(|p| if *p { '#' } else { ' ' })
                    .collect::<String>(),
            )
        }
    }) {
        println!("{}", &line);
    }
}

pub fn solve() {
    println!("part1: {}", part1(INPUT));
    part2(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const EXAMPLE1: &str = include_str!("day13-ex1.txt");
        assert_eq!(part1(EXAMPLE1), 17);
        part2(EXAMPLE1);
    }
}
