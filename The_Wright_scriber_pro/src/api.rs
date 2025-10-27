use reqwest::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;

/// Sends an email using the Resend.com API.
/// This function calls the Resend API to send an email.
/// api_key: Resend API key (starts with "re_")
/// from: sender email (must be verified in Resend)
/// to: recipient email
/// subject: email subject
/// body: HTML body of the email
pub async fn send_email(
    api_key: &str,
    from: &str,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    // Build JSON payload for Resend API
    let payload = json!({
        "from": from,
        "to": [to],
        "subject": subject,
        "html": body,
    });
    // Send POST request to Resend API
    let response = client
        .post("https://api.resend.com/emails")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?;
    if response.status().is_success() {
        Ok(())
    } else {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        Err(format!("Failed to send email: {} {}", status, text).into())
    }
}

/// Transcribe audio using the Gemini API.
/// This is still a stub implementation until Gemini integration is added.
pub async fn transcribe_via_gemini(
    api_key: &str,
    model: &str,
    content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: Implement API call to Gemini using reqwest
    Ok(String::new())
}
