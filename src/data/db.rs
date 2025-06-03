use rusqlite::{Connection, Result};

pub type DbConnection = Connection;
pub type DbResult<T> = Result<T>;

pub fn establish_connection() -> DbResult<DbConnection> {
    let conn = DbConnection::open("notes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            tags TEXT,
            create_at TEXT NOT NULL,
            update_at TEXT NOT NULL,
            is_pinned INTEGER DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}

