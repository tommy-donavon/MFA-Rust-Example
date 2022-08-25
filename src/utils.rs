use sha2::{Sha256, Digest};
use std::{env, str, error::Error};
use base64::encode;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::{Rng, thread_rng};

pub fn hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    encode(&result.to_vec())
}

pub fn gen_rand_num() -> String {
    thread_rng().gen_range(10000..=99999).to_string()
}

pub fn send_email(to_user: &str, code: &str) -> Result<(), Box<dyn Error>> {
    let host = env::var("SMTP_HOST")?;
    let username = env::var("SMTP_USERNAME")?;
    let password = env::var("SMTP_PASSWORD")?;
    let email = Message::builder()
        .from(username.parse()?)
        .to(to_user.parse()?)
        .subject("Security Code")
        .body(String::from(code))?;
    let creds = Credentials::new(username, password);
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(creds)
        .build();
    mailer.send(&email)?;
    Ok(())
}