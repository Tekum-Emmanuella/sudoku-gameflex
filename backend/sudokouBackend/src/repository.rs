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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        // Arrange
        let db_path = ":memory:"; // Use an in-memory database for testing
        let mut repo = SudokuSessionRepository::new(db_path).unwrap();
        let session = SudokuSession {
            id: 1,
            player_name: "Arthur".to_string(),
            game_state: "serialized_game_state".to_string(),
        };

        // Act
        let result = repo.create_session(&session);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_session() {
        // Arrange
        let db_path = ":memory:"; // Use an in-memory database for testing
        let mut repo = SudokuSessionRepository::new(db_path).unwrap();
        let session = SudokuSession {
            id: 1,
            player_name: "Arthur".to_string(),
            game_state: "serialized_game_state".to_string(),
        };
        repo.create_session(&session).unwrap();

        // Act
        let loaded_session = repo.get_session(1).unwrap();

        // Assert
        assert!(loaded_session.is_some());
        let loaded_session = loaded_session.unwrap();
        assert_eq!(loaded_session.id, session.id);
        assert_eq!(loaded_session.player_name, session.player_name);
        assert_eq!(loaded_session.game_state, session.game_state);
    }
}
