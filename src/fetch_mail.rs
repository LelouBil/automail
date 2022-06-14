use crate::credentials::ImapSettings;

pub fn fetch_mail(settings: ImapSettings) -> anyhow::Result<Option<String>> {
    let client = imap::ClientBuilder::new(settings.host, settings.port).native_tls()?;

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(settings.auth.username, settings.auth.password)
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    let mailbox = imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    
    match mailbox.unseen { 
        Some(latest_message_id) => {
            let messages = imap_session.fetch(latest_message_id.to_string(), "RFC822")?;
            let message = if let Some(m) = messages.iter().next() {
                m
            } else {
                return Ok(None);
            };

            // extract the message's body
            let body = message.body().expect("message did not have a body!");
            let body = std::str::from_utf8(body)
                .expect("message was not valid utf-8")
                .to_string();

            // be nice to the server and log out
            imap_session.logout()?;

            Ok(Some(body))
        }
        None => Ok(None)
    }
}