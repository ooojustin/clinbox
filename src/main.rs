extern crate lazy_static;
extern crate clipboard;

mod mail;
mod args;

use anyhow::{Result, anyhow};
use clipboard::{ClipboardContext, ClipboardProvider};
use mail::{Inbox, Email};
use std::sync::Mutex;
use std::thread;
use std::time::{Instant, Duration};
use lazy_static::lazy_static;
use clap::Parser;
use args::CLI;

lazy_static! {
    static ref INBOX: Mutex<Inbox> = Mutex::new(Inbox::new());
}

#[tokio::main]
async fn main() {

    let args = CLI::parse();

    // If inbox is being deleted, run delete func before calling establish_address
    if let args::Commands::Delete { copy: _ } = args.command {
        match Inbox::delete() {
            Ok(()) => {
                println!("Inbox deleted successfully.");
            },
            Err(err) => {
                eprintln!("Failed to delete inbox: {}", err);
            }
        }
    }

    let mut inbox = INBOX.lock().unwrap();

    if let Err(e) = inbox.establish_address().await {
        panic!("Failed to establish disposable mail session: {}", e);
    }

    let ai = inbox.address_info.as_ref().unwrap();
    let mut copy_email = false;

    match args.command {
        args::Commands::Show { count, copy }=> {
            println!("Email address: {}", ai.email);
            if copy {
                copy_email = true;
            }
            let all_emails: Vec<Email> = inbox.get_mail().await.unwrap();
            let emails: Vec<&Email> = all_emails.iter().take(count).collect();
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
        },
        args::Commands::Next { timeout, interval } => {
            println!("Email address: {}", ai.email);
            println!("Timeout duration: {} seconds", timeout);
            println!("Waiting to automatically open next received email...");

            let mut timed_out = false;
            let mut idx = 0;

            let interval_dur = Duration::from_secs(interval);
            let timeout_dur = Duration::from_secs(timeout);
            let start_time = Instant::now();
            let start_count = inbox.get_mail().await.unwrap().len();

            loop {
                let count = inbox.get_mail().await.unwrap().len();
                if count > start_count {
                    idx = (count - start_count) - 1;
                    break;
                }

                let elapsed_time = start_time.elapsed();
                if elapsed_time > timeout_dur {
                    timed_out = true;
                    break;
                }

                thread::sleep(interval_dur);
            }

            if !timed_out {
                let mut emails: Vec<Email> = inbox.get_mail().await.unwrap();
                let email = emails.get_mut(idx).unwrap();

                match inbox.populate_content(email).await {
                    Ok(()) => {
                        println!("Email received: {} (From: {})", email.subject, email.from);
                        email.open().unwrap();
                    },
                    Err(err) => {
                        eprintln!("Failed to populate email [ID: {}] content: {}", email.id, err);
                    }
                }
            }
        },
        args::Commands::Copy => {
            println!("Email address: {}", ai.email);
            copy_email = true;
        },
        args::Commands::Delete { copy } => { 
            println!("New email address created: {}", ai.email);
            if copy {
                copy_email = true;
            }
        }
    }

    if copy_email {
        match copy_to_clipboard(&ai.email) {
            Ok(()) => {
                println!("Email address copied to clipboard.");
            },
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    inbox.save_cookies();

}

fn print_emails(emails: &Vec<&Email>) {
    for email in emails {
        let r = if email.read { "○" } else { "●" };
        println!("[{}] {} {} - {} ({})", email.id, r, email.subject, email.from, email.when);
    }
}

fn copy_to_clipboard(text: &str) -> Result<()> {
    let mut clipboard: ClipboardContext = match ClipboardProvider::new() {
        Ok(clipboard) => clipboard,
        Err(err) => {
            return Err(anyhow!(format!("Failed to created instance of clipboard provider: {}", err)));
        }
    };

    if let Err(err) = clipboard.set_contents(text.to_owned()) {
        return Err(anyhow!(format!("Failed to set clipboard contents: {}", err)));
    }

    Ok(())
}
