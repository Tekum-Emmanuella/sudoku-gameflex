mod grid;

use grid::{SudokuGrid, SudokuError};

fn main() {
    // Example usage:
    let mut sudoku = SudokuGrid::new();
    sudoku.set_number(0, 0, 5).unwrap();
    sudoku.set_number(0, 2, 8).unwrap();
    sudoku.set_number(2, 0, 3).unwrap();
    sudoku.set_number(8, 8, 1).unwrap();

    // Serialize the SudokuGrid to JSON
    let serialized = serde_json::to_string(&sudoku).unwrap();
    println!("Serialized:\n{}", serialized);

    // Deserialize the JSON back into a SudokuGrid
    let deserialized: SudokuGrid = serde_json::from_str(&serialized).unwrap();
    deserialized.print_grid();
}
