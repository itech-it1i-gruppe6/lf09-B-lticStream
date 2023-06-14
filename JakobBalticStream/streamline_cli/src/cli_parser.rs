use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Device {
        #[arg(long)]
        ip: Option<String>
    },
    Clients,
    HostCount,
}
