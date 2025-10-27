use rusqlite::{Connection, Result};

pub struct Database;

impl Database {
    pub fn connect(db_path: &str) -> Result<Connection> {
        Connection::open(db_path)
    }

    pub fn init_transcription_logs(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS transcription_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                log_time TEXT NOT NULL,
                file_title TEXT NOT NULL,
                file_checksum TEXT NOT NULL,
                file_path TEXT NOT NULL,
                error_message TEXT,
                status TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn init_support_tickets(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS support_tickets (
                ticket_id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                user_email TEXT NOT NULL,
                subject TEXT NOT NULL,
                body TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn init_backup_history(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS backup_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                job_time TEXT NOT NULL,
                job_type TEXT NOT NULL,
                description TEXT,
                file_path TEXT NOT NULL,
                status TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
}
