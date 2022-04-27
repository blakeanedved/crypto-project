use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use std::io::{Read, Write};

//use crate::rsa::{RSAPubKey, rsa_key_gen, rsa_encrypt};
use crate::Args;

macro_rules! stream_read_size {
    ($stream:ident($size:expr) -> $data:ident $block:block) => {
        match $stream.read(&mut $data) {
            Ok(size) => {
                if size != $size {
                    eprintln!("An error occurred, terminating connection with {}\nError: invalid size for data stream", $stream.peer_addr().unwrap());
                    $stream.shutdown(Shutdown::Both).unwrap();
                    return;
                } else {
                    $block
                }
            }
            Err(e) => {
                eprintln!("An error occurred, terminating connection with {}\nError: {}", $stream.peer_addr().unwrap(), e);
                $stream.shutdown(Shutdown::Both).unwrap();
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
    let mut data = [0 as u8; 100];

    let (n_size, e_size) = stream_read_size!(stream(8) -> data {
        (as_u32_be(&data[0..4]), as_u32_be(&data[4..8]))
    });

    let n = stream_read_size!(stream(n_size as usize) -> data { 
        
    });
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
