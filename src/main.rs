extern crate core;


use anyhow::anyhow;
use clap::Parser;
use mail_parser::{HeaderValue, Message};



mod credentials;
mod fetch_mail;
mod send_mail;

const CONFIG_FILE: &str = "config.json";


fn main() {
    let config = credentials::read_config(CONFIG_FILE)
        .unwrap_or_else(|_| panic!("Erreur lors de l'ouverture du fichier de configuration {CONFIG_FILE}"));

    let result = fetch_mail::fetch_mail(config.imap).unwrap_or_else(|_| panic!("Erreur lors de la récupération des mails"));

    if let Some(mail) = result {
        match check_mail(mail).expect("Erreur lors de la vérification du corps de l'email") {
            None => {
                println!("Le dernier mail n'est pas bon");
            }
            Some(email) => {
                println!("Le dernier mail envoyé par {email} est bon");
                send_mail::send_answer(config.smtp, config.email, email);
            }
        }
    }else{
        println!("Aucun mail en attente");
    }
}

fn check_mail(mail: String) -> anyhow::Result<Option<String>> {
    // println!("Corp : {mail}");
    let message = Message::parse(mail.as_bytes());
    if let Some(parsed_mail) = message {
        let x = match parsed_mail.get_from() {
            HeaderValue::Address(addr) => addr.address.clone(),
            _ => None,
        }.ok_or_else(|| anyhow!("Erreur lors de la récupération de l'objet"))?;
        // println!("Envoyeur : {}", x);
        // println!("Sujet : {}", parsed_mail.get_subject().ok_or(anyhow!("Erreur lors de la récupération de l'objet"))?);
        let corp = parsed_mail.get_text_body(0).ok_or_else(|| anyhow!("Erreur lors de la récupération du corps"))?;
        // println!("Corps : {}", corp);
        if corp.contains("112684010") {
            Ok(Some(x.as_ref().to_string()))
        } else {
            Ok(None)
        }
    } else {
        println!("Aucun mail !");
        Ok(None)
    }
}
