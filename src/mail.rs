#[path = "cookies.rs"]
mod cookies;

#[path = "utils.rs"]
mod utils;

use std::fs::File;
use std::env::temp_dir;
use std::io::Write;
use uuid::Uuid;
use serde::Deserialize;
use anyhow::Result;
use reqwest::Client;
use reqwest_cookie_store::CookieStoreMutex;

const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com";

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
        match cookies::save_store(c) {
            Ok(()) => println!("Saved cookies."),
            Err(e) => eprintln!("Error saving cookies: {}", e)
        }
    }
}

impl Email {
    #[allow(dead_code)]
    pub fn open(&self) -> Result<()> {

        // random temp file for email html content
        let file_name = format!("{}.html", Uuid::new_v4());
        let mut file_path = temp_dir();
        file_path.push(file_name);
        
        // create file and write contents
        let mut file = File::create(&file_path)?;
        file.write_all(self.content.as_bytes())?;

        // convert file path to string, open in browser
        let path_string = file_path
            .to_string_lossy()
            .to_string();
        utils::open(path_string);

        Ok(())
    }

    fn list_from_str(text: String) -> Result<Vec<Self>> {
        let mut mail: Vec<Email> = serde_json::from_str(&text)?;
        for m in &mut mail {
            m.read = m.read_str.to_lowercase() != "new";
        }
        Ok(mail)
    }

    fn has_content(&self) -> bool {
        return !self.content.is_empty();
    }
}

pub struct Inbox {
    pub address_info: Option<AddressInfo>,
    client: Client,
    cookies: std::sync::Arc<CookieStoreMutex>,
}

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    pub email: String,

    #[serde(rename = "heslo")]
    _password: String,
}

#[derive(Debug, Deserialize)]
pub struct Email {
    pub id: u32,

    #[serde(rename = "predmet")]
    pub subject: String,

    #[serde(rename = "od")]
    pub from: String,

    #[serde(rename = "kdy")]
    pub when: String,

    #[serde(skip)]
    pub read: bool,

    #[serde(skip)]
    pub content: String,

    #[serde(rename = "precteno")]
    read_str: String,
}
