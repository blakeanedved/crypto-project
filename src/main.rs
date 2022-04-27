mod aes;
mod rsa;
mod server;
mod client;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, default_value_t = 3333)]
    port: u32,
}

#[derive(Subcommand)]
enum Commands {
    Client,
    Server,
    Test
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Client => client::client(args)?,
        Commands::Server => server::start(args)?,
        Commands::Test => utils::test()
    }

    Ok(())
}
