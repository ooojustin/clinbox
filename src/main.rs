mod mail;

extern crate lazy_static;

use mail::{Inbox, Email};
use tokio::time::{Duration, sleep};
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;

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

    //inbox.print_cookies();

    let interval = Duration::from_secs(3);
    sleep(interval).await;

    let mut emails: Vec<Email> = inbox.get_mail().await.unwrap();

    println!("emails: {:?}", emails);

    populate_emails(&mut inbox, &mut emails).await;
    
    println!("emails [populated]: {:?}", emails);

    inbox.save_cookies();

}

async fn populate_emails<'a>(
    inbox: &'a MutexGuard<'a, Inbox>, 
    emails: &mut Vec<Email>
) {
    for email in emails {
        if let Err(err) = inbox.populate_content(email).await {
            eprintln!("failed to populate email [ID: {}] content: {}", email.id, err);
        }
    }
}
