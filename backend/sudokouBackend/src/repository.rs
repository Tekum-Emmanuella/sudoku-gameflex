// repository.rs
use rusqlite::Result;
use serde::{Deserialize, Serialize}; // Import Serialize and Deserialize

// Define SudokuSession struct
#[derive(Debug, Serialize, Deserialize)] // Use Serialize and Deserialize macros
pub struct SudokuSession {
    pub id: i64,
    pub player_name: String,
    pub game_state: String,
}

// Implement the repository struct
pub struct SudokuSessionRepository {
    conn: rusqlite::Connection,
}

impl SudokuSessionRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = rusqlite::Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sudoku_sessions (
                 id INTEGER PRIMARY KEY,
                 player_name TEXT NOT NULL,
                 game_state TEXT NOT NULL
             )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn create_session(&mut self, session: &SudokuSession) -> Result<()> {
        self.conn.execute(
            "INSERT INTO sudoku_sessions (player_name, game_state)
             VALUES (?1, ?2)",
            [&session.player_name, &session.game_state],
        )?;
        Ok(())
    }

    pub fn get_session(&self, session_id: i64) -> Result<Option<SudokuSession>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, player_name, game_state FROM sudoku_sessions WHERE id = ?1")?;
        let mut rows = stmt.query([&session_id])?;
        if let Some(row) = rows.next()? {
            let session = SudokuSession {
                id: row.get(0)?,
                player_name: row.get(1)?,
                game_state: row.get(2)?,
            };
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    // Other CRUD operations can be implemented here
}
