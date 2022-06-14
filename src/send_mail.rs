use std::fmt::format;
use std::fs;
use std::path::Path;
use crate::credentials::{MailSettings, SmtpSettings};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Attachment, header, MultiPart, MultiPartKind, SinglePart};
use lettre::message::header::ContentType;
use lettre::transport::smtp::client::Tls;
use mail_parser::{Addr, HeaderValue};


pub fn send_answer(settings: SmtpSettings, mail_settings: MailSettings, email: String) {
    let email = Message::builder()
        .from(format!("{} <{}>", mail_settings.from_name, mail_settings.from_address).parse().unwrap())
        .to(format!("Toi <{}>",email).parse().unwrap())
        .subject(mail_settings.subject)
        .multipart(MultiPart::builder().kind(MultiPartKind::Mixed)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(mail_settings.text)
            )
            .singlepart(
                Attachment::new(mail_settings.attachment.clone())
                    .body(fs::read(mail_settings.attachment.as_str()).expect("Erreur de lecture du fichier")
                          , ContentType::parse("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet").unwrap())
            )
        )
        .unwrap();

    let creds = Credentials::new(settings.auth.username, settings.auth.password);

// Open a remote connection to gmail
    let mailer = SmtpTransport::starttls_relay(settings.host.as_str())
        .unwrap()
        .port(settings.port)
        .credentials(creds)
        .build();

// Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}