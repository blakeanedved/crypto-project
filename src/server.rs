use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{Read, Write};
use rug::Integer;
use rug::integer::Order;

use crate::Args;
use crate::rsa::{RSAPublicKey, rsa_encrypt};

#[macro_export]
macro_rules! stream_read {
    ($stream:ident($size:ident) -> $data:ident $block:block) => {
        use std::net::{Shutdown, TcpStream};
        use std::io::Read;
        match $stream.read(&mut $data) {
            Ok($size) => $block
            Err(e) => {
                eprintln!("An error occurred, terminating connection with {}\nError: {}", $stream.peer_addr().unwrap(), e);
                $stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
        }
    };

    ($stream:ident -> $data:ident $block:block) => {
        match $stream.read(&mut $data) {
            Ok(_) => $block
            Err(e) => {
                eprintln!("An error occurred, terminating connection with {}\nError: {}", $stream.peer_addr().unwrap(), e);
                $stream.shutdown(std::net::Shutdown::Both).unwrap();
                return;
            }
        }
    }
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 600];

    let (n, e) = stream_read!(stream -> data {
        let n_size = as_u32_be(&data[0..4]) as usize;
        let e_size = as_u32_be(&data[4..8]) as usize;

        let n = Integer::from_digits(&data[8..8+n_size], Order::Msf);
        let e = Integer::from_digits(&data[8+n_size..8+n_size+e_size], Order::Msf);

        (n, e)
    });

    let rsa_pub_key = RSAPublicKey::new(n, e);

    println!("RSA key retrival successful: {:?}", rsa_pub_key);

    let message = Integer::from(43);

    let enc_key = rsa_encrypt(&rsa_pub_key, message);
    
    stream.write(&enc_key.to_digits::<u8>(Order::Msf)[..]).unwrap();
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
