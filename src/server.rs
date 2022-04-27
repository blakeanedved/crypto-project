use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{Read, Write};

use crypto_bigint::U1024;

//use crate::rsa::{RSAPubKey, rsa_key_gen, rsa_encrypt};
use crate::Args;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];

    loop {
        match stream.read(&mut data) {
            Ok(size) => {
                break;
            }
            Err(e) => {
                eprintln!("An error occurred, terminating connection with {}\nError: {}", stream.peer_addr().unwrap(), e);
                stream.shutdown(Shutdown::Both).unwrap();

                break;
            }
        }
    }
}

pub fn start(args: Args) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port))?;

    println!("Server listening on port {}", args.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr()?);

                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("failed connection: {}", e);
            }
        }
    }

    Ok(())
}
