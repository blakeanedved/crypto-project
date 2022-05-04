use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

use crate::aes::AES;
use crate::rsa::{rsa_encrypt, RSAPublicKey};
use crate::Args;

#[macro_export]
macro_rules! stream_read {
    ($stream:ident($size:ident) -> $data:ident $block:block) => {
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
                $stream.shutdown(Shutdown::Both).unwrap();
                return;
            }
        }
    }
}

fn as_u32_be(array: &[u8]) -> u32 {
    ((array[0] as u32) << 24)
        + ((array[1] as u32) << 16)
        + ((array[2] as u32) << 8)
        + ((array[3] as u32) << 0)
}

fn handle_client(mut stream: TcpStream, args: Args) {
    let stream_id = stream.peer_addr().unwrap();
    let mut data = [0 as u8; 600];

    let rsa_pub_key = stream_read!(stream -> data {
        let n_size = as_u32_be(&data[0..4]) as usize;
        let e_size = as_u32_be(&data[4..8]) as usize;

        RSAPublicKey::from_bytes(&data[8..8+n_size], &data[8+n_size..8+n_size+e_size])
    });

    if args.debug {
        println!("e={}\nn={}", &rsa_pub_key.e, &rsa_pub_key.n);
    }

    if args.debug {
        println!(
            "{}: RSA key retrival successful: {}",
            stream_id, rsa_pub_key
        );
    } else {
        println!("{}: RSA key retrival successful", stream_id);
    }

    let aes = AES::new();

    if args.debug {
        println!("{}: {}", stream_id, aes);
    }

    let message = aes.key_to_vec();

    let enc_key = rsa_encrypt(&rsa_pub_key, &message[..]);

    stream.write(&enc_key[..]).unwrap();

    loop {
        stream_read!(stream(size) -> data {
            if args.debug {
                println!("{}: enc_msg={:?}", stream_id, &data[0..size]);
            }
            let dec_msg = aes.decrypt(&data[0..size]);
            if args.debug {
                println!("{}: dec_msg={:?}", stream_id, &dec_msg[..]);
            }
            let message = String::from_utf8_lossy(&dec_msg[..]).to_string();
            if message.len() == 0 { break; }

            println!("{}: Recieved message: {}", stream_id, message);

            let enc = aes.encrypt(&message.as_bytes()[..]);
            stream.write(&enc[..]).unwrap();
        });
    }

    println!("{}: Connection terminated", stream_id);
}

pub fn start(args: Args) -> anyhow::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port))?;

    println!("Server listening on port {}", args.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr()?);
                let args = args.clone();
                thread::spawn(move || {
                    handle_client(stream, args);
                });
            }
            Err(e) => {
                eprintln!("failed connection: {}", e);
            }
        }
    }

    Ok(())
}
