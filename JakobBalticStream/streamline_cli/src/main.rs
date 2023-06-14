use anyhow;
use clap::Parser;
use cli_parser::{Cli, Commands};
use streamline_cisco::{client::CiscoClient, auth::LoginCredentials};

mod cli_parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let mut client = CiscoClient::new();
    client.login(LoginCredentials { username: args.username, password: args.password }).await?;
    match &args.command {
        Commands::Clients => println!("{:#?}", client.get_all_clients().await?),
        Commands::HostCount => println!("{:#?}", client.get_host_count().await?),
        Commands::Device { ip } => {
            match ip {
                Some(ip) => println!("{:#?}", client.get_device_by_ipv4(&ip).await?),
                None => println!("{:#?}", client.get_all_devices().await?),
            }
        },
    }
    Ok(())
}
