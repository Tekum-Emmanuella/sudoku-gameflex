mod grid;

use grid::{SudokuGrid, SudokuError};

fn main() {
    let mut sudoku = SudokuGrid::new();
    sudoku.set_number(0, 0, 5).unwrap();
    sudoku.set_number(0, 2, 8).unwrap();
    sudoku.set_number(2, 0, 3).unwrap();
    sudoku.set_number(8, 8, 1).unwrap();

    sudoku.print_grid();
}