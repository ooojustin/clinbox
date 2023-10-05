mod mail;

#[tokio::main]
async fn main() {

    let mut inbox = mail::Inbox::new();

    match inbox.establish_address().await {
        Ok(()) => {},
        Err(e) => {
            eprintln!("error: {}", e);
        }
    };

    println!("{}", inbox.address);

}
