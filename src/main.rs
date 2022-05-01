mod aes;
mod rsa;
mod server;
mod client;
mod utils;
mod miller_rabin;

use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[clap(author, version)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short, long, default_value_t = 3333)]
    port: u32,

    #[clap(short, long)]
    debug: bool,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    Client {
        #[clap(long, default_value = "127.0.0.1")]
        host: String
    },
    Server,
    Test
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Client { .. } => client::client(args)?,
        Commands::Server => server::start(args)?,
        Commands::Test => utils::test()
    }

    Ok(())
}
