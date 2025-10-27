use reqwest::Client;
use serde_json::json;

/// Sends an email using the Resend.com API.
///
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
    let payload = json!({
        "from": from,
        "to": [to],
        "subject": subject,
        "html": body,
    });

    let response = client
        .post("https://api.resend.com/emails")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let text = response.text().await?;
        Err(format!("Failed to send email: {}", text).into())
    }
}

/// Placeholder for Gemini API transcription; not implemented.
pub async fn transcribe_via_gemini(
    _api_key: &str,
    _content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // TODO: integrate with Gemini API for transcription if desired.
    Ok(String::new())
}
