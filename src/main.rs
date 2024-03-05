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
    match board.empty_cells.last().copied() {
        None => Some(board),
        Some((row, col)) => board
            .valid_moves(row, col)
            .find_map(|value| solve_recursive(board.make_move(row, col, value))),
    }
}

fn main() {
    // let grid = [
    //     [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 0, 3, 0, 8, 5],
    //     [0, 0, 1, 0, 2, 0, 0, 0, 0],
    //     [0, 0, 0, 5, 0, 7, 0, 0, 0],
    //     [0, 0, 4, 0, 0, 0, 1, 0, 0],
    //     [0, 9, 0, 0, 0, 0, 0, 0, 0],
    //     [5, 0, 0, 0, 0, 0, 0, 7, 3],
    //     [0, 0, 2, 0, 1, 0, 0, 0, 0],
    //     [0, 0, 0, 0, 4, 0, 0, 0, 9],
    // ];

    let grid = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let start_time = std::time::Instant::now();
    let solution = solve(grid).unwrap();
    let elapsed = start_time.elapsed();

    println!("Found solution in {:#?}", elapsed);
    for row in solution.iter() {
        println!("{:?}", row);
    }
}
