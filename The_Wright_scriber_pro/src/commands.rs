use tauri::State;
use crate::auth::{login_user, change_password};
use crate::db::Database;

/// Tauri command to log in a user.
#[tauri::command]
pub async fn cmd_login(
    email: String,
    password: String,
    db: State<'_, Database>,
) -> Result<bool, String> {
    match login_user(&db, &email, &password).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

/// Tauri command to change a user's password.
#[tauri::command]
pub async fn cmd_change_password(
    email: String,
    old_password: String,
    new_password: String,
    db: State<'_, Database>,
) -> Result<(), String> {
    match change_password(&db, &email, &old_password, &new_password).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// Additional commands would be added here for transcription, export, logs, tickets, etc.
