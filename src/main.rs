extern crate lazy_static;

mod mail;
mod args;

use mail::{Inbox, Email};
use std::sync::Mutex;
use lazy_static::lazy_static;
use clap::Parser;
use args::CLI;

lazy_static! {
    static ref INBOX: Mutex<Inbox> = Mutex::new(Inbox::new());
}

#[tokio::main]
async fn main() {

    let mut inbox = INBOX.lock().unwrap();

    match inbox.establish_address().await {
        Ok(()) => {
            let ai = inbox.address_info.as_ref().unwrap();
            println!("email: {}", ai.email);
        },
        Err(e) => {
            eprintln!("error: {}", e);
        }
    };

    let args = CLI::parse();
    match args.command {
        args::Commands::Show { count }=> {
            let ai = inbox.address_info.as_ref().unwrap();
            let all_emails: Vec<Email> = inbox.get_mail().await.unwrap();
            let emails: Vec<&Email> = all_emails.iter().take(count).collect();
            println!("Email address: {}", ai.email);
            print_emails(&emails);
        },
        args::Commands::Open { id }  => {
            let mut emails: Vec<Email> = inbox.get_mail().await.unwrap();
            match emails.iter_mut().find(|email| email.id == id) {
                Some(email) => {
                    match inbox.populate_content(email).await {
                        Ok(()) => {
                            email.open().unwrap();
                        },
                        Err(err) => {
                            eprintln!("Failed to populate email [ID: {}] content: {}", email.id, err);
                        }
                    }
                },
                None => {
                    println!("Failed to find email with specified ID: {}", id);
                }
            }
        }
    }

    inbox.save_cookies();

}

fn print_emails(emails: &Vec<&Email>) {
    for email in emails {
        let r = if email.read { "⦾" } else { "⦿" };
        println!("[{}] {} {} - {} ({})", email.id, r, email.subject, email.from, email.when);
    }
}
