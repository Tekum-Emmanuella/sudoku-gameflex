use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SudokuGrid {
    data: [[u8; 9]; 9], // 9x9 grid to hold Sudoku numbers
}

impl SudokuGrid {
    fn new() -> Self {
        SudokuGrid { data: [[0; 9]; 9] }
    }

    fn set_number(&mut self, row: usize, col: usize, number: u8) -> Result<(), &'static str> {
        if !Self::is_valid_position(row, col) {
            return Err("Invalid position");
        }
        self.data[row][col] = number;
        Ok(())
    }

    fn get_number(&self, row: usize, col: usize) -> Option<u8> {
        if Self::is_valid_position(row, col) {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    fn is_valid_position(row: usize, col: usize) -> bool {
        row < 9 && col < 9
    }

    fn print_grid(&self) {
        for row in &self.data {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}

fn main() {
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
