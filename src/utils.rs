use sha2::{Sha256, Digest};
use std::{env, str};
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

pub fn gen_rand_num() -> String{
    let num = thread_rng().gen_range(0..=10000);
    num.to_string()
}

pub fn send_email(to_user: &str, code: &str) -> Result<(), lettre::transport::smtp::Error> {
    let host = env::var("SMTP_HOST").unwrap();
    let username = env::var("SMTP_USERNAME").unwrap();
    let password = env::var("SMTP_PASSWORD").unwrap();
    let email = Message::builder()
        .from(username.parse().unwrap())
        .to(to_user.parse().unwrap())
        .subject("Security Code")
        .body(String::from(code))
        .unwrap();
    let creds = Credentials::new(username, password);
    let mailer = SmtpTransport::relay(host.as_str())
        .unwrap()
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(e)
    }
}