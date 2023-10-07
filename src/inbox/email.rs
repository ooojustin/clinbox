use std::fs::File;
use std::env::temp_dir;
use std::io::Write;
use uuid::Uuid;
use serde::Deserialize;
use anyhow::{Result, anyhow};

#[path = "utils.rs"]
mod utils;

/// Represents an email.
#[derive(Debug, Deserialize)]
pub struct Email {
    /// The unique identifier of the email.
    pub id: u32,

    /// The subject of the email.
    #[serde(rename = "predmet")]
    pub subject: String,

    /// The sender of the email.
    #[serde(rename = "od")]
    pub from: String,

    /// Relative time of when the email was received.
    #[serde(rename = "kdy")]
    pub when: String,

    /// Indicates whether or not the email has been read.
    #[serde(skip)]
    pub read: bool,

    /// The content of the email. (HTML)
    #[serde(skip)]
    pub content: String,

    /// A string representation of the 'read' field.
    #[serde(rename = "precteno")]
    read_str: String,
}

impl Email {
    /// Write the email HTML contents in a temporary file and open the email in the default browser.
    pub fn open(&self) -> Result<()> {
        if !self.has_content() {
            return Err(anyhow!(format!("Failed to open email [id: {}] because it is missing content.", self.id)));
        }

        // random temp file for email html content
        let file_name = format!("{}.html", Uuid::new_v4());
        let mut file_path = temp_dir();
        file_path.push(file_name);


        // add subject/from address to top of email content
        let from_html = self.from.replace("<", "&lt;").replace(">", "&gt;");
        let email_html = format!("
            <center>
                <h1 style=\"margin-bottom: 0px\">{}</h1>
                <h2 style=\"margin-top: 0px\">{}</h2>
                <a 
                    style=\"text-decoration: none; font-weight: 660; color: #f0f;\" 
                    href=\"https://github.com/ooojustin/clinbox\"
                >
                    [email retrieved by clinbox]
                </a>
            </center>\n{}
        ", self.subject, from_html, self.content);
        
        // create file and write contents
        let mut file = File::create(&file_path)?;
        file.write_all(email_html.as_bytes())?;

        // convert file path to string, open in browser
        let path_string = file_path
            .to_string_lossy()
            .to_string();
        utils::open(path_string);

        Ok(())
    }

    /// Returns whether or not the email content has been set.
    pub fn has_content(&self) -> bool {
        return !self.content.is_empty();
    }

    /// Create a list of emails from a JSON serialized input string.
    pub fn list_from_str(text: String) -> Result<Vec<Self>> {
        let mut mail: Vec<Email> = serde_json::from_str(&text)?;
        for m in &mut mail {
            m.read = m.read_str.to_lowercase() != "new";
        }
        Ok(mail)
    }
}

pub trait EmailVector {
    /// Print the list of emails to the console.
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
