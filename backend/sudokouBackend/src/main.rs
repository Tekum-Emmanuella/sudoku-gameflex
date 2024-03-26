mod grid;
mod repository;

use grid::SudokuGrid;
use repository::SudokuSessionRepository; // Import directly

use rusqlite::Result;

fn main() -> Result<()> {
    // Sudoku session repository operations
    let db_path = "sudoku_sessions.db";
    let mut repo = SudokuSessionRepository::new(db_path)?;

    let session = repository::SudokuSession {
        id: 1,
        player_name: "Alice".to_string(),
        game_state: "serialized_game_state".to_string(),
    };

    repo.create_session(&session)?;

    if let Some(loaded_session) = repo.get_session(1)? {
        println!("Loaded session: {:?}", loaded_session);
    }

    // Sudoku grid operations
    let mut sudoku = SudokuGrid::new();
    sudoku.set_number(0, 0, 5).unwrap();
    sudoku.set_number(0, 2, 8).unwrap();
    sudoku.set_number(2, 0, 3).unwrap();
    sudoku.set_number(8, 8, 1).unwrap();

    sudoku.print_grid();

    Ok(())
}
