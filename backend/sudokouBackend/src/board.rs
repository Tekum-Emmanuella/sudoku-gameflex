#[derive(Clone, Debug)]
pub struct SudokuGrid {
    data: [[Option<u8>; 9]; 9], // 9x9 grid to hold Sudoku numbers
}

impl SudokuGrid {
    pub fn new() -> Self {
        SudokuGrid { data: [[None; 9]; 9] }
    }

    pub fn set_number(&mut self, row: usize, col: usize, number: u8) -> Result<(), SudokuError> {
        if !Self::is_valid_position(row, col) {
            return Err(SudokuError::InvalidPosition);
        }

        if number < 1 || number > 9 {
            return Err(SudokuError::InvalidNumber);
        }

        self.data[row][col] = Some(number);
        Ok(())
    }

    pub fn get_number(&self, row: usize, col: usize) -> Option<u8> {
        if Self::is_valid_position(row, col) {
            self.data[row][col]
        } else {
            None
        }
    }

    pub fn is_valid_position(row: usize, col: usize) -> bool {
        row < 9 && col < 9
    }

    pub fn print_grid(&self) {
        for row in &self.data {
            for &cell in row {
                match cell {
                    Some(value) => print!("{} ", value),
                    None => print!("- "),
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
pub enum SudokuError {
    InvalidPosition,
    InvalidNumber,
}