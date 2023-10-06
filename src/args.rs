use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Show {
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
    Open {
        #[arg()]
        id: u32,
    }
}
