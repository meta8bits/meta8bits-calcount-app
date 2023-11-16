use anyhow::Result;
#[cfg(feature = "enable_smtp_email")]
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
#[cfg(feature = "enable_smtp_email")]
use std::env;

#[cfg(feature = "enable_smtp_email")]
pub async fn send_email(to: &str, subject: &str, msg: &str) -> Result<()> 