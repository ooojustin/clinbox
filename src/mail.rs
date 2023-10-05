use reqwest::{Client, header};

use serde::Deserialize;

const DISPOSABLE_MAIL: &str = "https://www.disposablemail.com/";
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0";

pub struct Inbox {
    client: Client,
    pub address: String,
}

impl Inbox {
    pub fn new() -> Inbox {
        Inbox {
            client: Client::new(),
            address: "".to_string(),
        }
    }

    pub async fn establish_address(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.get(DISPOSABLE_MAIL)
            .send()
            .await?;

        let response = self.client.get(format!("{}index/index", DISPOSABLE_MAIL))
            .headers(headers(true))
            .send()
            .await?;

        if response.status().is_success() {
            let text = response.text().await?;
            let data: AddressInfo = serde_json::from_str(&text)?;
            //println!("{:?}", data);
            self.address = data.email.to_string();
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Request failed with status code: {}", response.status()),
            )));
        }

        Ok(())
    }
}

pub fn headers(xml_http: bool) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));
    if xml_http {
        headers.insert("X-Requested-With", header::HeaderValue::from_static("XMLHttpRequest"));
    }
    headers
}

#[derive(Debug, Deserialize)]
struct AddressInfo {
    email: String,

    #[serde(rename = "heslo")]
    _password: String,
}
