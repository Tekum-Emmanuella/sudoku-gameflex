#[derive(Debug, Clone, Copy)]
struct SudokuGrid {
    grid: [[u8; 9]; 9],
}

impl SudokuGrid {
    fn new() -> Self {
        SudokuGrid { grid: [[0; 9]; 9] }
    }

    fn set_value(&mut self, row: usize, col: usize, value: u8) -> Result<(), &'static str> {
        if value < 1 || value > 9 {
            return Err("Value must be between 1 and 9");
        }
        self.grid[row][col] = value;
        Ok(())
    }

    fn get_value(&self, row: usize, col: usize) -> u8 {
        self.grid[row][col]
    }

    fn is_valid(&self) -> bool {
        for i in 0..9 {
            if !self.is_valid_row(i) || !self.is_valid_col(i) || !self.is_valid_square(i) {
                return false;
            }
        }
        true
    }

    fn is_valid_row(&self, row: usize) -> bool {
        let mut seen = [false; 9];
        for col in 0..9 {
            let value = self.grid[row][col];
            if value != 0 {
                if seen[(value - 1) as usize] {
                    return false;
                }
                seen[(value - 1) as usize] = true;
            }
        }
        true
    }

    fn is_valid_col(&self, col: usize) -> bool {
        let mut seen = [false; 9];
        for row in 0..9 {
            let value = self.grid[row][col];
            if value != 0 {
                if seen[(value - 1) as usize] {
                    return false;
                }
                seen[(value - 1) as usize] = true;
            }
        }
        true
    }

    fn is_valid_square(&self, square: usize) -> bool {
        let start_row = (square / 3) * 3;
        let start_col = (square % 3) * 3;
        let mut seen = [false; 9];
        for row_offset in 0..3 {
            for col_offset in 0..3 {
                let value = self.grid[start_row + row_offset][start_col + col_offset];
                if value != 0 {
                    if seen[(value - 1) as usize] {
                        return false;
                    }
                    seen[(value - 1) as usize] = true;
                }
            }
        }
        true
    }

    fn display(&self) {
        for row in &self.grid {
            println!("{:?}", row);
        }
    }
}

fn main() {
    let mut sudoku = SudokuGrid::new();
    sudoku.set_value(0, 0, 5).unwrap();
    sudoku.set_value(0, 1, 3).unwrap();
    sudoku.set_value(0, 4, 7).unwrap();
    sudoku.set_value(1, 0, 6).unwrap();
    sudoku.set_value(1, 3, 1).unwrap();
    sudoku.set_value(1, 4, 9).unwrap();
    sudoku.set_value(1, 5, 5).unwrap();
    sudoku.set_value(2, 1, 9).unwrap();
    sudoku.set_value(2, 2, 8).unwrap();
    sudoku.set_value(2, 7, 6).unwrap();
    sudoku.set_value(3, 0, 8).unwrap();
    sudoku.set_value(3, 4, 6).unwrap();
    sudoku.set_value(3, 8, 3).unwrap();
    sudoku.set_value(4, 0, 4).unwrap();
    sudoku.set_value(4, 3, 8).unwrap();
    sudoku.set_value(4, 5, 3).unwrap();
    sudoku.set_value(4, 8, 1).unwrap();
    sudoku.set_value(5, 0, 7).unwrap();
    sudoku.set_value(5, 4, 2).unwrap();
    sudoku.set_value(5, 8, 6).unwrap();
    sudoku.set_value(6, 1, 6).unwrap();
    sudoku.set_value(6, 6, 2).unwrap();
    sudoku.set_value(6, 7, 8).unwrap();
    sudoku.set_value(7, 3, 4).unwrap();
    sudoku.set_value(7, 4, 1).unwrap();
    sudoku.set_value(7, 5, 9).unwrap();
    sudoku.set_value(7, 8, 5).unwrap();
    sudoku.set_value(8, 4, 8).unwrap();
    sudoku.set_value(8, 7, 7).unwrap();
    sudoku.display();

    if sudoku.is_valid() {
        println!("Sudoku grid is valid!");
    } else {
        println!("Sudoku grid is invalid!");
    }
}
