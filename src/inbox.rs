#[path = "cookies.rs"]
mod cookies;
mod utils;
pub mod email;

use serde::Deserialize;
use anyhow::Result;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use email::Email;

const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com";

pub struct Inbox {
    pub address_info: Option<AddressInfo>,
    client: Client,
    cookies: std::sync::Arc<CookieStoreMutex>,
}

impl Inbox {
    pub fn new() -> Self {
        let mutex = cookies::get_store_mutex();
        let cookies = std::sync::Arc::new(mutex);

        let client = Client::builder()
            .cookie_provider(std::sync::Arc::clone(&cookies))
            .build()
            .unwrap();

        Inbox {
            address_info: None,
            client,
            cookies,
        }
    }

    pub async fn establish_address(&mut self) -> Result<()> {
        self.client.get(DISPOSABLE_MAIL)
            .headers(utils::headers(false))
            .send()
            .await?;

        let response = self.client.get(format!("{}/index/index", DISPOSABLE_MAIL))
            .headers(utils::headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let ai: AddressInfo = serde_json::from_str(&text)?;

        self.address_info = Some(ai);

        Ok(())
    }

    pub async fn get_mail(&self) -> Result<Vec<Email>> {
        let response = self.client.get(format!("{}/index/refresh", DISPOSABLE_MAIL))
            .headers(utils::headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let mail = Email::list_from_str(text)?;
        
        Ok(mail)
    }

    pub async fn populate_content(&self, mail: &mut Email) -> Result<()> {
        if mail.has_content() {
            return Ok(());
        }

        let response = self.client.get(format!("{}/email/id/{}", DISPOSABLE_MAIL, mail.id))
            .headers(utils::headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        mail.content = text;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn print_cookies(&self) {
        let store = self.cookies.lock().unwrap();
        for c in store.iter_any() {
            println!("Cookies: {:?}", c);
        }
    }

    pub fn save_cookies(&self) {
        let c = std::sync::Arc::clone(&self.cookies);
        if let Err(err) = cookies::save_store(c) {
            eprintln!("Error saving cookies: {}", err);
        }
    }

    pub fn delete() -> Result<()> {
        cookies::delete_file()?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    pub email: String,

    #[serde(rename = "heslo")]
    _password: String,
}
