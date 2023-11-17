use anyhow::Result;
#[cfg(feature = "enable_smtp_email")]
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
#[cfg(feature = "enable_smtp_email")]
use std::env;

#[cfg(feature = "enable_smtp_email")]
pub async fn send_email(to: &str, subject: &str, msg: &str) -> Result<()> {
    let email = Message::builder()
        .from("beancount.bot <jdevries3133@gmail.com>".parse()?)
        .reply_to("beancount.bot <jdevries3133@gmail.com>".parse()?)
        .to(to.parse()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(msg))?;

    let username = env::var("SMTP_EMAIL_USERNAME")?;
    le