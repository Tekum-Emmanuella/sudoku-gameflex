#[derive(Clone, Debug)]
pub struct SudokuGrid {
    data: [[Option<u8>; 9]; 9], // 9x9 grid to hold Sudoku numbers
}

impl SudokuGrid {
    pub fn new() -> Self {
        SudokuGrid {
            data: [[None; 9]; 9],
        }
    }

    pub fn set_number(&mut self, row: usize, col: usize, number: u8) -> Result<(), SudokuError> {
        if !Self::is_valid_position(row, col) {
            return Err(SudokuError::InvalidPosition);
        }

        if !(1..=9).contains(&number) {
            return Err(SudokuError::InvalidNumber);
        }

        self.data[row][col] = Some(number);
        Ok(())
    }

    pub fn get_number(&self, row: usize, col: usize) -> Result<Option<u8>, SudokuError> {
        if !Self::is_valid_position(row, col) {
            return Err(SudokuError::InvalidPosition);
        }

        Ok(self.data[row][col])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_number() {
        let mut sudoku = SudokuGrid::new();
        assert!(sudoku.set_number(0, 0, 5).is_ok());
        assert!(sudoku.set_number(0, 2, 8).is_ok());
        assert!(sudoku.set_number(2, 0, 3).is_ok());
        assert!(sudoku.set_number(8, 8, 1).is_ok());
        assert!(sudoku.set_number(9, 9, 9).is_err());
        assert!(sudoku.set_number(0, 0, 0).is_err());
    }

    #[test]
    fn test_get_number() {
        let mut sudoku = SudokuGrid::new();
        sudoku.set_number(0, 0, 5).unwrap();
        sudoku.set_number(0, 2, 8).unwrap();
        sudoku.set_number(2, 0, 3).unwrap();
        sudoku.set_number(8, 8, 1).unwrap();

        assert_eq!(sudoku.get_number(0, 0).unwrap(), Some(5));
        assert_eq!(sudoku.get_number(0, 2).unwrap(), Some(8));
        assert_eq!(sudoku.get_number(2, 0).unwrap(), Some(3));
        assert_eq!(sudoku.get_number(8, 8).unwrap(), Some(1));
        assert_eq!(sudoku.get_number(1, 1).unwrap(), None);
        assert!(sudoku.get_number(9, 9).is_err());
    }
}
