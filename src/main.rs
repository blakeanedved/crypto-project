mod aes;
mod client;
#[cfg(target_os = "unix")]
mod miller_rabin;
#[cfg(target_os = "windows")]
mod miller_rabin_win;
mod rsa;
mod server;
mod utils;

extern crate num_bigint_dig as num_bigint;

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
        host: String,
    },
    Server,
    Test,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Client { .. } => client::client(args)?,
        Commands::Server => server::start(args)?,
        Commands::Test => utils::test(),
    }

    Ok(())
}
