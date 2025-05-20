use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::error::Error;

pub struct GmailConfig {
    pub username: String,
    pub password: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub async fn send(config: GmailConfig) -> Result<(), Box<dyn Error>> {
    // Gmail SMTP Server
    let smtp_server = "smtp.gmail.com";

    // Email message
    let email = Message::builder()
        .from(config.from.parse()?)
        .to(config.to.parse()?)
        .subject(config.subject)
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(config.body)?;

    // Set up SMTP client with TLS
    let creds = Credentials::new(config.username, config.password);
    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
    // match mailer.send(&email) {
    //     Ok(_) => println!("Email sent successfully!"),
    //     Err(e) => eprintln!("Failed to send email: {:?}", e),
    // }
}
