use tauri::State;
use crate::auth::{login_user, change_password};
use crate::db::Database;
use crate::api::send_email;
use uuid::Uuid;
use chrono::Local;

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

#[tauri::command]
pub async fn cmd_submit_ticket(
    user_email: String,
    message: String,
    db: State<'_, Database>,
) -> Result<String, String> {
    let ticket_id = format!("T-{}", Uuid::new_v4());
    let created_at = Local::now().format("%d/%m/%Y %H:%M:%S").to_string();
    let subject = format!("Support Ticket {}", ticket_id);
    // store in DB
    if let Err(e) = db.add_support_ticket(&ticket_id, &created_at, &user_email, &subject, &message, "Open") {
        return Err(format!("Database error: {}", e));
    }
    // send email
    let api_key = std::env::var("RESEND_API_KEY").unwrap_or_default();
    let from_email = std::env::var("SUPPORT_FROM").unwrap_or_default();
    let to_email = std::env::var("SUPPORT_TO").unwrap_or_default();
    match send_email(&api_key, &from_email, &to_email, &subject, &message).await {
        Ok(_) => Ok(ticket_id),
        Err(e) => Err(format!("{}", e)),
    }
}

#[tauri::command]
pub fn cmd_get_tickets(db: State<'_, Database>) -> Result<Vec<crate::schema::Ticket>, String> {
    match db.get_all_tickets() {
        Ok(tickets) => Ok(tickets),
        Err(e) => Err(e.to_string()),
    }
}
