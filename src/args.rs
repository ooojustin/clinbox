use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Show inbox information and list emails.
    Show {
        /// Number of emails to list.
        #[arg(short, long, default_value = "5")]
        count: usize,
        /// Copy email address to clipboard.
        #[arg(short = 'C', long)]
        copy: bool,
    },
    /// Open a specific email by providing the ID.
    Open {
        /// The ID of the email.
        #[arg()]
        id: u32,
    },
    /// Wait for a new email to be received and automatically open it.
    Next {
        /// The maximum amount of time to wait in seconds.
        #[arg(short, long, default_value = "120")]
        timeout: u64,
        /// The interval in between refreshing emails in seconds.
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
    /// Copy email address to clipboard.
    Copy,
    /// Delete the current inbox and automatically generate new email address.
    Delete {
        /// Copy new email address to clipboard.
        #[arg(short, long)]
        copy: bool,
    },
    /// Display the duration until the current inbox expires.
    Expires,
    /// Open the website that this program uses behind the scenes.
    Website,
    /// Open the GitHub repository for this application.
    Github,
}
