mod grid;
mod instance;

use grid::SudokuGrid;
use instance::{Board, SIZE};
use std::io::{self, Write};

fn main() {
    let mut board = Board::new();
    board.generate();
    board.print();

    while !board.is_complete() {
        let mut input = String::new();
        print!("Enter move (row col value): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<usize> = input
            //.trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if parts.len() != 3 {
            println!("Invalid input! Please enter row col value (e.g., 1 2 3).");
            continue;
        }

        let row = parts[0];
        let col = parts[1];
        let value = parts[2];

        if row >= SIZE || col >= SIZE || !(1..=9).contains(&value) {
            board.make_move(row, col, value as u8);
            board.print();
        }

        println!("Congratulations! You solved the Sudoku puzzle!");

        // Integrate the SudokuGrid logic from the second main function
        let mut sudoku = SudokuGrid::new();
        sudoku.set_number(0, 0, 5).unwrap();
        sudoku.set_number(0, 2, 8).unwrap();
        sudoku.set_number(2, 0, 3).unwrap();
        sudoku.set_number(8, 8, 1).unwrap();

        sudoku.print_grid();
    }
}
