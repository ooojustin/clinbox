mod mail;

use mail::{Inbox, Email};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {

    let mut inbox = Inbox::new();

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

    let email = emails.get_mut(0).unwrap();
    
    if let Err(err) = inbox.populate_content(email).await {
        eprintln!("failed to populate email content: {}", err);
    }
    
    println!("emails: {:?}", emails);

    inbox.save_cookies();

}
