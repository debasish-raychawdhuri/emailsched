use base64::{
    engine::{general_purpose, Config},
    Engine as _,
};
use imap::{Client, Session};
use native_tls::TlsStream;
use regex::Replacer;
use std::{error::Error, net::TcpStream};

#[derive(Eq, PartialEq, Debug)]
enum TransferEncoding {
    Base64,
    Plain,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Email {
    pub subject: String,
    pub body: String,
}
fn get_subject_from_header(header: &str) -> String {
    let mut subject = String::new();
    let mut in_subject = false;
    for line in header.split("\n") {
        if line.starts_with("Subject") {
            let parts = line.split(":").collect::<Vec<&str>>();
            if parts.len() > 1 {
                subject.push_str(parts[1].trim());
                in_subject = true;
            }
        } else if in_subject && line.starts_with(" ") {
            subject.push_str(line.trim());
        } else {
            in_subject = false;
        }
    }
    subject
}

fn get_content_transfer_encoding_from_header(header: &str) -> TransferEncoding {
    let mut encoding = "plain";
    for line in header.split("\n") {
        if line.starts_with("Content-Transfer-Encoding") {
            let parts = line.split(":").collect::<Vec<&str>>();
            if parts.len() > 1 {
                encoding = parts[1].trim();
            }
        }
    }
    if encoding.contains("base64") {
        TransferEncoding::Base64
    } else {
        TransferEncoding::Plain
    }
}
pub fn fetch_inbox_top(
    config: &crate::config::Config,
) -> Result<Vec<Option<Email>>, Box<dyn Error>> {
    let client: Client<TlsStream<TcpStream>> =
        imap::ClientBuilder::new("imap.cse.iitb.ac.in", 993).native_tls()?;

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session: Session<TlsStream<TcpStream>> =
        client.login(&config.username, &config.password).unwrap();

    // we want to fetch the first email in the INBOX mailbox
    let mailbox = imap_session.select("INBOX")?;

    let last = mailbox.exists;
    //download 30 recent messages
    println!("{} messages in INBOX", last);
    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch(
        format!("{}:{}", last - 29, last),
        "(RFC822.TEXT RFC822.HEADER)",
    )?;

    let mut message_list = Vec::new();
    for message in messages.iter() {
        let body = message.text().ok_or("No body found")?.to_vec();
        let body = std::str::from_utf8(&body)
            .expect("message was not valid utf-8")
            .to_string();
        let header =
            String::from_utf8(message.header().ok_or("No header part found")?.to_vec()).unwrap();
        let body = if get_content_transfer_encoding_from_header(&header) == TransferEncoding::Base64
        {
            decode_base64(&body)
        } else {
            body
        };
        let subject = get_subject_from_header(&header);
        message_list.push(Some(Email { subject, body }));
    }
    dbg!(message_list.len());
    // extract the message's body
    // be nice to the server and log out
    imap_session.logout()?;

    Ok(message_list)
}
fn decode_base64(input: &str) -> String {
    let input = input.replace("\n", "").replace("\r", "").replace(" ", "");
    let data = general_purpose::STANDARD.decode(input.as_bytes()).unwrap();
    String::from_utf8(data).unwrap().to_string()
}
