use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::collections::BinaryHeap;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct EmptyCell {
    row: usize,
    col: usize,
    valid_moves: Vec<u8>,
}

impl Ord for EmptyCell {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min heap
        other.valid_moves.len().cmp(&self.valid_moves.len())
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: [[u8; 9]; 9],
    row_contains: [u16; 9],
    col_contains: [u16; 9],
    box_contains: [u16; 9],
    empty_cells: Vec<(usize, usize)>,
}

impl Board {
    fn new(grid: [[u8; 9]; 9]) -> Self {
        let mut board = Board {
            grid,
            row_contains: [0; 9],
            col_contains: [0; 9],
            box_contains: [0; 9],
            empty_cells: Vec::new(),
        };

        for row in 0..9 {
            for col in 0..9 {
                match board.grid[row][col] {
                    0 => board.empty_cells.push((row, col)),
                    value => {
                        let bit = 1 << (value - 1);
                        board.row_contains[row] |= bit;
                        board.col_contains[col] |= bit;
                        board.box_contains[get_box_index(row, col)] |= bit;
                    }
                }
            }
        }

        board
    }

    fn valid_moves(&self, row: usize, col: usize) -> impl Iterator<Item = u8> + '_ {
        let invalid_moves = self.row_contains[row]
            | self.col_contains[col]
            | self.box_contains[get_box_index(row, col)];
        (1..=9).filter(move |&value| {
            let bit = 1 << (value - 1);
            bit & invalid_moves == 0
        })
    }

    fn make_move(&self, row: usize, col: usize, value: u8) -> Self {
        let bit = 1 << (value - 1);
        let mut new_board = self.clone();
        new_board.grid[row][col] = value;
        new_board.row_contains[row] |= bit;
        new_board.col_contains[col] |= bit;
        new_board.box_contains[get_box_index(row, col)] |= bit;
        new_board.empty_cells.pop();
        new_board
    }
}

#[inline]
fn get_box_index(row: usize, col: usize) -> usize {
    (row / 3) * 3 + (col / 3)
}

fn solve(grid: [[u8; 9]; 9]) -> Option<[[u8; 9]; 9]> {
    let board = Board::new(grid);

    solve_recursive(board).map(|board| board.grid)
}

fn solve_recursive(board: Board) -> Option<Board> {
    match board.empty_cells.last() {
        None => Some(board.clone()),
        Some((row, col)) => board
            .valid_moves(*row, *col)
            .find_map(|value| solve_recursive(board.make_move(*row, *col, value))),
    }
}

fn parse_grid(input: &str) -> [[u8; 9]; 9] {
    let mut grid = [[0; 9]; 9];
    // Input is an 81 char sequence with 1-9 and . as empty cells
    for (i, c) in input.chars().enumerate() {
        let row = i / 9;
        let col = i % 9;
        grid[row][col] = match c {
            '1'..='9' => c.to_digit(10).unwrap() as u8,
            _ => 0,
        };
    }
    grid
}

fn main() {
    let default_grid = [
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 3, 0, 8, 5],
        [0, 0, 1, 0, 2, 0, 0, 0, 0],
        [0, 0, 0, 5, 0, 7, 0, 0, 0],
        [0, 0, 4, 0, 0, 0, 1, 0, 0],
        [0, 9, 0, 0, 0, 0, 0, 0, 0],
        [5, 0, 0, 0, 0, 0, 0, 7, 3],
        [0, 0, 2, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 4, 0, 0, 0, 9],
    ];

    let start_time = std::time::Instant::now();
    let solution = solve(default_grid).unwrap();
    let elapsed = start_time.elapsed();
    println!("Time: {:?}", elapsed);

    for row in solution.iter() {
        println!("{:?}", row);
    }

    // let default_grid = [
    //     [5, 3, 0, 0, 7, 0, 0, 0, 0],
    //     [6, 0, 0, 1, 9, 5, 0, 0, 0],
    //     [0, 9, 8, 0, 0, 0, 0, 6, 0],
    //     [8, 0, 0, 0, 6, 0, 0, 0, 3],
    //     [4, 0, 0, 8, 0, 3, 0, 0, 1],
    //     [7, 0, 0, 0, 2, 0, 0, 0, 6],
    //     [0, 6, 0, 0, 0, 0, 2, 8, 0],
    //     [0, 0, 0, 4, 1, 9, 0, 0, 5],
    //     [0, 0, 0, 0, 8, 0, 0, 7, 9],
    // ];

    let grids = include_str!("../sudoku10k.txt")
        .lines()
        .map(parse_grid)
        .collect::<Vec<_>>();

    let times: Vec<Duration> = grids
        .par_iter()
        .progress()
        .map(|&grid| {
            let start_time = std::time::Instant::now();
            let _ = solve(grid).unwrap();
            start_time.elapsed()
        })
        .collect();

    let total_time: Duration = times.iter().sum();
    let average_time = total_time / times.len() as u32;

    println!("Average time: {:?}", average_time);

    let max_time = times.iter().max().unwrap();
    let min_time = times.iter().min().unwrap();

    println!("Max time: {:?}", max_time);
    println!("Min time: {:?}", min_time);
}
