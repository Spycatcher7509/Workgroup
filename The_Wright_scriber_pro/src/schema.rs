use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub group: String,
    pub must_change_pass: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i64,
    pub log_time: String,
    pub file_title: String,
    pub file_checksum: String,
    pub file_path: String,
    pub error_message: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub ticket_id: String,
    pub created_at: String,
    pub user_email: String,
    pub subject: String,
    pub body: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupRecord {
    pub id: i64,
    pub job_time: String,
    pub job_type: String,
    pub description: Option<String>,
    pub file_path: String,
    pub status: String,
}
