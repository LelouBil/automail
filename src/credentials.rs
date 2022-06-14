use std::fs;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug,JsonSchema)]
pub struct Credentials {
    pub imap: ImapSettings,
    pub smtp: SmtpSettings,
    pub email: MailSettings,
}

#[derive(Deserialize, Debug,JsonSchema)]
pub struct MailSettings{
    pub from_name: String,
    pub from_address: String,
    pub subject: String,
    pub text: String,
    pub attachment: String
}

pub fn read_config(file_path: &str) -> Result<Credentials, anyhow::Error> {
    let file_content = fs::read_to_string(file_path)?;
    let config = serde_json::from_str::<Credentials>(file_content.as_str())?;
    Ok(config)
}

#[derive(Deserialize, Debug,JsonSchema)]
pub struct AuthInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug,JsonSchema)]
pub struct ImapSettings {
    pub host: String,
    pub port: u16,
    pub auth: AuthInfo,
}

#[derive(Deserialize, Debug,JsonSchema)]
pub struct SmtpSettings {
    pub host: String,
    pub port: u16,
    pub auth: AuthInfo,
} 