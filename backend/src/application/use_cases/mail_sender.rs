use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;
use std::env;

pub async fn send_verification_email(name: Option<String>, to: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(Mailbox::new(name, "noreply@example.com".into()))
        .to(Mailbox::new(name, to.into()))
        .subject("Email Verification")
        .body(format!(
            "Hi {}!\n Please verify your email by clicking the following link: http://example.com/verify_email?token={}",
            name.unwrap_or("User".to_string()),
            token
        ))?;

    let smtp_server = env::var("SMTP_SERVER")?;
    let smtp_port: u16 = env::var("SMTP_PORT")?.parse()?;
    let smtp_username = env::var("SMTP_USERNAME")?;
    let smtp_password = env::var("SMTP_PASSWORD")?;

    let smtp = SmtpTransport::relay(&smtp_server)
        .port(smtp_port)
        .credentials((smtp_username, smtp_password))
        .build();

    smtp.send(&email).await?;

    Ok(())
}
