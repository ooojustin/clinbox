#[path = "cookies.rs"]
mod cookies;

use serde::Deserialize;
use anyhow::Result;
use reqwest::{Client, header};
use reqwest_cookie_store::CookieStoreMutex;

const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com";

// Most common user agents: https://www.useragents.me/
// Chrome 117.0.0 [Windows]
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36";

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
            .headers(headers(false))
            .send()
            .await?;

        let response = self.client.get(format!("{}/index/index", DISPOSABLE_MAIL))
            .headers(headers(true))
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
            .headers(headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let mail = Email::list_from_str(text)?;
        
        Ok(mail)
    }

    pub async fn populate_content(&self, mail: &mut Email) -> Result<()> {
        if !mail.content.is_empty() {
            // content is already populated
            println!("CONTENT ALREADY POPULATED");
            return Ok(());
        }

        let response = self.client.get(format!("{}/email/id/{}", DISPOSABLE_MAIL, mail.id))
            .headers(headers(true))
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
    fn list_from_str(text: String) -> Result<Vec<Self>> {
        let mut mail: Vec<Email> = serde_json::from_str(&text)?;
        for m in &mut mail {
            m.read = m.read_str.to_lowercase() != "new";
        }
        Ok(mail)
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
    pub id: i32,

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

pub fn headers(xml_http: bool) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));
    if xml_http {
        headers.insert("X-Requested-With", header::HeaderValue::from_static("XMLHttpRequest"));
    }
    headers
}
