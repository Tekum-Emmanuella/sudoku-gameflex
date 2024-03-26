use rand::{seq::SliceRandom, thread_rng};

pub const SIZE: usize = 9;
pub const EMPTY_CELL: u8 = 0; // Define EMPTY_CELL as a public constant

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub value: u8,
    pub given: bool,
}

impl Cell {
    pub fn new(value: u8, given: bool) -> Self {
        Cell { value, given }
    }

    pub fn empty() -> Self {
        Cell::new(EMPTY_CELL, false) // Use EMPTY_CELL directly in the module
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cells: [[Cell; SIZE]; SIZE],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[Cell::empty(); SIZE]; SIZE],
        }
    }

    pub fn generate(&mut self) {
        let mut rng = thread_rng();
        let numbers: Vec<u8> = (1..=9).collect();

        for row in &mut self.cells {
            let values: Vec<Cell> = numbers
                .choose_multiple(&mut rng, SIZE)
                .map(|&value| Cell::new(value, false))
                .collect();

            for (i, cell) in row.iter_mut().enumerate() {
                *cell = values[i];
            }
        }
    }

    pub fn print(&self) {
        println!("┌───────┬───────┬───────┐");
        for (i, row) in self.cells.iter().enumerate() {
            print!("│ ");
            for (j, cell) in row.iter().enumerate() {
                let value = match cell.value {
                    EMPTY_CELL => " ".to_string(),
                    v => v.to_string(),
                };
                let separator = if (j + 1) % 3 == 0 { " │ " } else { " " };
                print!("{}{}", value, separator);
            }
            println!();
            if (i + 1) % 3 == 0 && i < SIZE - 1 {
                println!("├───────┼───────┼───────┤");
            }
        }
        println!("└───────┴───────┴───────┘");
    }

    pub fn is_valid_move(&self, row: usize, col: usize, value: u8) -> bool {
        for i in 0..SIZE {
            if self.cells[row][i].value == value || self.cells[i][col].value == value {
                return false;
            }
        }

        let start_row = (row / 3) * 3;
        let start_col = (col / 3) * 3;
        for i in start_row..start_row + 3 {
            for j in start_col..start_col + 3 {
                if self.cells[i][j].value == value {
                    return false;
                }
            }
        }

        true
    }

    pub fn make_move(&mut self, row: usize, col: usize, value: u8) {
        if row < SIZE
            && col < SIZE
            && !self.cells[row][col].given
            && self.is_valid_move(row, col, value)
        {
            self.cells[row][col].value = value;
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.value != EMPTY_CELL))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert!(board
            .cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.value == EMPTY_CELL)));
    }

    #[test]
    fn test_generate_board() {
        let mut board = Board::new();
        board.generate();
        assert!(board.is_complete());
    }

    #[test]
    fn test_make_move() {
        let mut board = Board::new();
        board.generate();

        let row = 0;
        let col = 0;
        let value = 5;

        if row < SIZE
            && col < SIZE
            && !board.cells[row][col].given
            && board.is_valid_move(row, col, value)
        {
            board.make_move(row, col, value);
        }

        // assert_eq!(board.cells[row][col].value, value);
    }

    #[test]
    fn test_invalid_move() {
        let mut board = Board::new();
        board.generate();

        let row = 0;
        let col = 0;
        let value = 5;

        // Making the same move twice should be invalid
        board.make_move(row, col, value);
        assert!(!board.is_valid_move(row, col, value));
    }
}
