use reqwest::{Client, header};

use serde::Deserialize;

const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0";

impl Inbox {
    pub fn new() -> Self {
        let client = Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        Inbox {
            client,
            address_info: None,
        }
    }

    pub async fn establish_address(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.get(DISPOSABLE_MAIL)
            .headers(headers(false))
            .send()
            .await?;

        let response = self.client.get(format!("{}index/index", DISPOSABLE_MAIL))
            .headers(headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let ai: AddressInfo = serde_json::from_str(&text)?;

        self.address_info = Some(ai);

        Ok(())
    }

    pub async fn get_mail(&self) -> Result<Vec<Email>, Box<dyn std::error::Error>> {
        let response = self.client.get(format!("{}index/refresh", DISPOSABLE_MAIL))
            .headers(headers(true))
            .send()
            .await?
            .error_for_status()?;

        let text = response.text().await?;
        let mail = Email::list_from_str(text)?;
        
        Ok(mail)
    }
}

impl Email {
    fn list_from_str(text: String) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut mail: Vec<Email> = serde_json::from_str(&text)?;
        for m in &mut mail {
            m.read = m.read_str.to_lowercase() != "new";
        }
        Ok(mail)
    }
}

pub struct Inbox {
    client: Client,
    pub address_info: Option<AddressInfo>,
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
