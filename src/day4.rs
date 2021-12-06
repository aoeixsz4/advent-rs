use std::{io, convert::TryFrom};
use crate::input;

struct Cell {
    value: i64,
    marked: bool
}

impl TryFrom<&str> for Cell {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = s.parse() {
            Ok(Cell { value, marked: false })
        } else {
            Err("Could not parse number")
        }
    }
}

type Row = Vec<Cell>;
type Board = Vec<Row>;

fn mark(board: &mut Board, n: i64) {
    for row in board {
        for cell in row {
            if cell.value == n { cell.marked = true }
        }
    }
}

fn is_winner(board: &Board) -> bool {
    let len = board[0].len();
    for row in board {
        let mut marked_count = 0;
        for cell in row {
            if cell.marked { marked_count += 1; }
        }
        if marked_count == len { return true; }
    }
    let height = board.len();
    for j in 0 .. len {
        let mut marked_count: usize = 0;
        for i in 0 .. height {
            if board[i][j].marked == true { marked_count += 1; }
        }
        if marked_count == height { return true; }
    }
    false
}

fn count_score(board: &Board) -> i64 {
    let mut score = 0;
    for row in board {
        for cell in row {
            if !cell.marked { score += cell.value; }
        }
    }
    score
}

fn common_prep(data: &[String]) -> (Vec<i64>, Vec<Board>) {
    let mut data_iterator = data.split(|s| s.eq(""));
    let bingo_draws = data_iterator.next().unwrap()[0].split(",").map(|s|s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let boards = data_iterator.map(|board_slice|{
        board_slice.iter().map(|row_string|{
            row_string.split_whitespace().map(|cell_string| Cell::try_from(cell_string).unwrap()).collect::<Row>()
        }).collect::<Board>()
    }).collect::<Vec<Board>>();
    (bingo_draws, boards)
}

fn part1(data: &[String]) -> i64 {
    let (bingo_draws, mut boards) = common_prep(data);
    for n in bingo_draws {
        for mut board in &mut boards {
            mark(&mut board, n);
            if is_winner(board) {
                return count_score(board) * n;
            }
        }
    }
    0
}

fn part2(data: &[String]) -> i64 {
    let (bingo_draws, mut boards) = common_prep(data);
    let mut last_winning_score = 0;
    for n in bingo_draws {
        let mut len = boards.len();
        let mut i = 0;
        while i < len {
            mark(&mut boards[i], n);
            if is_winner(&boards[i]) {
                last_winning_score = count_score(&boards[i]) * n;
                boards.remove(i);
                len -= 1;
            } else {
                i += 1;
            }
        }
    }
    last_winning_score
}

pub fn solve() -> Result<(), io::Error> {
    let data = input::get_lines_input("day4")
        .expect("couldn't open input file for day4 (should be inputs/day4)");
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
    Ok(())
}