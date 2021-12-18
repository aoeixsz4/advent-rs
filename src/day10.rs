const INPUT: &str = include_str!("day10.txt");

fn score_line(line: &str) -> i64 {
    let mut open_token_stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => open_token_stack.push('('),
            '[' => open_token_stack.push('['),
            '{' => open_token_stack.push('{'),
            '<' => open_token_stack.push('<'),
            ')' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '(' {
                    return 3;
                }
            }
            ']' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '[' {
                    return 57;
                }
            }
            '}' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '{' {
                    return 1197;
                }
            }
            '>' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '<' {
                    return 25137;
                }
            }
            _ => unreachable!(),
        }
    }
    0
}

fn get_completion_string(line: &str) -> Option<String> {
    let mut open_token_stack = Vec::new();
    for c in line.chars() {
        match c {
            '(' => open_token_stack.push('('),
            '[' => open_token_stack.push('['),
            '{' => open_token_stack.push('{'),
            '<' => open_token_stack.push('<'),
            ')' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '(' {
                    return None;
                }
            }
            ']' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '[' {
                    return None;
                }
            }
            '}' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '{' {
                    return None;
                }
            }
            '>' => {
                let last_token = open_token_stack.pop().unwrap();
                if last_token != '<' {
                    return None;
                }
            }
            _ => unreachable!(),
        }
    }
    Some(
        open_token_stack
            .iter()
            .rev()
            .map(|c| match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            })
            .collect::<String>(),
    )
}

fn score_completion_string(comp: &str) -> i64 {
    comp.chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        })
        .fold(0, |acc, x| acc * 5 + x)
}

fn part1(data: &[&str]) -> i64 {
    data.iter().map(|l| score_line(l)).sum()
}

fn part2(data: &[&str]) -> i64 {
    let mut scores = data
        .iter()
        .filter_map(|l| get_completion_string(l))
        .map(|completion| score_completion_string(&completion))
        .collect::<Vec<i64>>();
    scores.sort_unstable();
    let len = scores.len();
    if len % 2 != 1 {
        panic!("number of scores isn't odd!");
    }
    scores[(len - 1) / 2]
}

pub fn solve() {
    let data: Vec<&str> = INPUT.lines().collect();
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const EXAMPLE1: &str = include_str!("day10-ex1.txt");
        let data: Vec<&str> = EXAMPLE1.lines().collect();
        assert_eq!(part1(&data), 26397);
        assert_eq!(part2(&data), 288957);
    }
}
