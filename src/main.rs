mod mail;

use mail::Inbox;
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

    let interval = Duration::from_secs(30);
    sleep(interval).await;

    loop {
        match inbox.get_mail().await {
            Ok(mail) => {
                println!("{:?}", mail);
            },
            Err(e) => {
                eprintln!("mail error: {}", e);
            }
        };
        sleep(interval).await;
    }

}
