use std::fs::File;
use std::env::temp_dir;
use std::io::Write;
use uuid::Uuid;
use serde::Deserialize;
use anyhow::Result;

#[path = "utils.rs"]
mod utils;

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

impl Email {
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

    pub fn list_from_str(text: String) -> Result<Vec<Self>> {
        let mut mail: Vec<Email> = serde_json::from_str(&text)?;
        for m in &mut mail {
            m.read = m.read_str.to_lowercase() != "new";
        }
        Ok(mail)
    }

    pub fn has_content(&self) -> bool {
        return !self.content.is_empty();
    }
}

pub trait EmailVector {
    fn print(&self);
}

impl EmailVector for Vec<&Email> {
    fn print(&self) {
        for email in self {
            let r = if email.read { "○" } else { "●" };
            println!("[{}] {} {} - {} ({})", email.id, r, email.subject, email.from, email.when);
        }
    }
}
