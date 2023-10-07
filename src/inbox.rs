#[path = "cookies.rs"]
mod cookies;
mod date_time;
pub mod utils;
pub mod email;

use serde::Deserialize;
use anyhow::Result;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;
use email::Email;
use chrono::{Duration, DateTime, Utc};

pub const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com";

/// A disposable email inbox.
pub struct Inbox {
    /// Inbox email address information.
    pub address_info: Option<AddressInfo>,
    /// Reqwest client containing inbox session data.
    client: Client,
    /// Cookie store mutex accessible in thread safe fashion.
    cookies: std::sync::Arc<CookieStoreMutex>,
}

impl Inbox {
    /// Create an inbox using the default cookies retrieved from disk.
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

    /// Establish new disposable mail session if it does not exist,
    /// and assign AddressInfo object (including email address) in Inbox.
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

    /// Retrieve up-to-date list of emails in this disposable inbox.
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

    /// Populate content of a specified email.
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
    
    /// Determine how long it will be until this inbox expires.
    pub async fn get_exp_delta(&self) -> Result<Duration> {
        let response = self.client.get(format!("{}/index/zivot", DISPOSABLE_MAIL))
            .headers(utils::headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let li: Lifetime = serde_json::from_str(&text)?;

        Ok(li.end - li.now)
    }

    /// Print cookies stored in Inbox.
    #[allow(dead_code)]
    pub fn print_cookies(&self) {
        let ai = self.address_info.as_ref().unwrap();
        println!("=== inbox {} cookies ===", ai.email);
        let store = self.cookies.lock().unwrap();
        for cookie in store.iter_any() {
            println!("{:?}", cookie);
        }
    }

    /// Save inbox cookies to disk so the session can be restored.
    pub fn save_cookies(&self) {
        let c = std::sync::Arc::clone(&self.cookies);
        if let Err(err) = cookies::save_store(c) {
            eprintln!("Error saving cookies: {}", err);
        }
    }

    /// Delete cookies file from disk so a new session can be created.
    pub fn delete() -> Result<()> {
        cookies::delete_file()?;
        Ok(())
    }
}

/// Address information of an inbox.
#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    /// Disposable email address of the inbox.
    pub email: String,

    /// Randomized password generated for the inbox.
    #[serde(rename = "heslo")]
    _password: String,
}

// Disposable inbox session lifetime information.
#[derive(Debug, Deserialize)]
struct Lifetime {
    /// The timestamp at which this object was retrieved.
    #[serde(rename = "ted", with = "date_time")]
    now: DateTime<Utc>,

    /// The timestamp at which the associated inbox expires.
    #[serde(rename = "konec", with = "date_time")]
    end: DateTime<Utc>,
}
