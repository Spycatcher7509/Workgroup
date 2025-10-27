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

/// Adds a new support ticket to the database
    pub fn add_support_ticket(
        conn: &Connection,
        ticket_id: &str,
        created_at: &str,
        user_email: &str,
        subject: &str,
        body: &str,
        status: &str,
    ) -> Result<()> {
        conn.execute(
            "INSERT INTO support_tickets (ticket_id, created_at, user_email, subject, body, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (ticket_id, created_at, user_email, subject, body, status),
        )?;
        Ok(())
    }

    /// Retrieves all support tickets from the database
    pub fn get_all_tickets(conn: &Connection) -> Result<Vec<(String, String, String, String, String, String)>> {
        let mut stmt = conn.prepare(
            "SELECT ticket_id, created_at, user_email, subject, body, status FROM support_tickets ORDER BY created_at DESC",
        )?;
        let ticket_iter = stmt.query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?;
        let mut tickets = Vec::new();
        for t in ticket_iter {
            tickets.push(t?);
        }
        Ok(tickets)
    }
    }
}
