use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::SinglePart;
use std::env;

pub async fn send_reset_email(email: &str, reset_link: &str) -> Result<(), Error> {
    let smtp_server: &str = &env::var("SMTP_SERVER").expect("環境変数 `SMTP_SERVER` は設定する必要があります。");
    let smtp_port: u16 = env::var("SMTP_PORT")
        .unwrap_or_else(|_| "587".to_string())
        .parse()
        .expect("環境変数 `SMTP_PORT` は設定する必要があります。");
    let smtp_username = env::var("SMTP_USERNAME").expect("環境変数 `SMTP_USERNAME` は設定する必要があります。");
    let smtp_password = env::var("SMTP_PASSWORD").expect("環境変数 `SMTP_PASSWORD` は設定する必要があります。");

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    // メールの内容を作成
    let email_message = Message::builder()
        .from("no-reply@example.com".parse().unwrap()) // 送信者のメールアドレス
        .to(email.parse().unwrap()) // 受信者のメールアドレス
        .subject("Password Reset Request")
        .singlepart(SinglePart::plain(format!(
            "Please click the following link to reset your password: {}",
            reset_link
        )))?;


    // SMTP接続の設定と送信
    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .port(smtp_port)
        .build();

    mailer.send(&email_message).await?;

    Ok(())
}
