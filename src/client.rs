use std::net::TcpStream;
use rug::integer::Order;
use std::io::{Write, Read};

use crate::Args;
use crate::rsa::{rsa_key_gen, rsa_decrypt};

pub fn client(args: Args) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect(format!("localhost:{}", args.port))?;

    let aes_key_size = 128;

    let (rsa_pub_key, rsa_priv_key) = rsa_key_gen();

    let n = rsa_pub_key.n.to_digits::<u8>(Order::Msf);
    let e = rsa_pub_key.e.to_digits::<u8>(Order::Msf);

    let n_size = n.len() as u32;
    let e_size = e.len() as u32;

    let data: [u8; 8] = unsafe { std::mem::transmute([n_size.to_be(), e_size.to_be()]) };

    stream.write(&data)?;
    stream.write(&n[..])?;
    stream.write(&e[..])?;

    let mut data = [0 as u8; 32];

    stream.read(&mut data)?;

    let aes_key = rsa_decrypt(&rsa_priv_key, &data[0..(aes_key_size / 8)]);

    eprintln!("Message: {}", aes_key);

    Ok(())
}
