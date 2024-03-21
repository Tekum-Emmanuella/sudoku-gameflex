use rand::{thread_rng, Rng};
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

struct SudokuGridFactory;

impl SudokuGridFactory {
    fn generate_grid(difficulty_level: u8) -> SudokuGrid {
        let mut grid = SudokuGrid::new();
        SudokuGridFactory::fill_grid(&mut grid, difficulty_level);
        grid
    }

    fn fill_grid(grid: &mut SudokuGrid, difficulty_level: u8) {
        let mut rng = thread_rng();
        let cells_to_fill = match difficulty_level {
            1 => 30, // Easy level
            2 => 25, // Medium level
            3 => 20, // Hard level
            _ => 30, // Default to easy level
        };

        for _ in 0..cells_to_fill {
            let row = rng.gen_range(0..9);
            let col = rng.gen_range(0..9);
            let num = rng.gen_range(1..=9); // 1 to 9 (inclusive)
            if grid.get_number(row, col).is_none() && SudokuGrid::is_valid_position(row, col) {
                grid.set_number(row, col, num).unwrap();
            }
        }
    }
}

fn main() {
    println!("Choose a difficulty level:");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");
    let difficulty_level: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Defaulting to Easy level.");
            1
        }
    };

    let grid = SudokuGridFactory::generate_grid(difficulty_level);

    println!("Generated Sudoku Grid:");
    grid.print_grid();
}
