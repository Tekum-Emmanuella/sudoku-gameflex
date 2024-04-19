use rand::{seq::SliceRandom, thread_rng, Rng};

// Define the Sudoku grid structure
#[derive(Clone, Debug)]
pub struct SudokuGrid {
    data: [[Option<u8>; 9]; 9],
}

// Error enum for Sudoku operations
#[derive(Debug)]
pub enum SudokuError {
    InvalidPosition,
    InvalidNumber,
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

// Enum to represent game levels
#[derive(Debug)]
pub enum GameLevel {
    Easy,
    Medium,
    Hard,
}

// Factory struct to generate Sudoku grids based on game levels
pub struct SudokuGridFactory;

impl SudokuGridFactory {
    pub fn generate_grid(difficulty: GameLevel) -> SudokuGrid {
        let mut grid = SudokuGrid::new();
        match difficulty {
            GameLevel::Easy => SudokuGridFactory::generate_easy_grid(&mut grid),
            GameLevel::Medium => SudokuGridFactory::generate_medium_grid(&mut grid),
            GameLevel::Hard => SudokuGridFactory::generate_hard_grid(&mut grid),
        }
        grid
    }

    fn generate_easy_grid(grid: &mut SudokuGrid) {
        let mut rng = thread_rng();
        // Generate an easy level Sudoku grid by removing numbers
        SudokuGridFactory::remove_numbers(grid, &mut rng, 40); // Adjust the number of removed numbers as needed
    }

    fn generate_medium_grid(grid: &mut SudokuGrid) {
        let mut rng = thread_rng();
        // Generate a medium level Sudoku grid by removing numbers
        SudokuGridFactory::remove_numbers(grid, &mut rng, 50); // Adjust the number of removed numbers as needed
    }

    fn generate_hard_grid(grid: &mut SudokuGrid) {
        let mut rng = thread_rng();
        // Generate a hard level Sudoku grid by removing numbers
        SudokuGridFactory::remove_numbers(grid, &mut rng, 60); // Adjust the number of removed numbers as needed
    }

    // Helper function to remove numbers from the solved grid to generate different difficulty levels
    fn remove_numbers(grid: &mut SudokuGrid, rng: &mut impl Rng, num_to_remove: usize) {
        let mut rng_indices: Vec<(usize, usize)> = (0..81).map(|i| (i / 9, i % 9)).collect();
        rng_indices.shuffle(rng);

        for (row, col) in rng_indices.into_iter().take(num_to_remove) {
            grid.data[row][col] = None;
        }
    }
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

    #[test]
    fn test_generate_grid_easy() {
        let grid = SudokuGridFactory::generate_grid(GameLevel::Easy);
        // Add assertions or print statements to verify the generated grid for easy difficulty
    }

    #[test]
    fn test_generate_grid_medium() {
        let grid = SudokuGridFactory::generate_grid(GameLevel::Medium);
        // Add assertions or print statements to verify the generated grid for medium difficulty
    }

    #[test]
    fn test_generate_grid_hard() {
        let grid = SudokuGridFactory::generate_grid(GameLevel::Hard);
        // Add assertions or print statements to verify the generated grid for hard difficulty
    }
}
