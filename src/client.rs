use std::net::TcpStream;
use rug::integer::Order;
use std::io::{Write, Read};

use crate::{Args, aes::AES, Commands};
use crate::rsa::{rsa_key_gen, rsa_decrypt};

pub fn client(args: Args) -> anyhow::Result<()> {
    let host = match args.command {
        Commands::Client { host } => host,
        _ => unreachable!()
    };

    println!("Connecting to {}:{}", host, args.port);

    let mut stream = TcpStream::connect(format!("{}:{}", host, args.port))?;

    println!("Connection successful");
    println!("Generating RSA keypair");

    let (rsa_pub_key, rsa_priv_key) = rsa_key_gen();

    if args.debug {
        println!("{}\n{}", rsa_pub_key, rsa_priv_key);
    }

    println!("Exchanging RSA key");

    let n = rsa_pub_key.n.to_digits::<u8>(Order::Msf);
    let e = rsa_pub_key.e.to_digits::<u8>(Order::Msf);

    let n_size = n.len() as u32;
    let e_size = e.len() as u32;

    let data: [u8; 8] = unsafe { std::mem::transmute([n_size.to_be(), e_size.to_be()]) };

    let mut rsa_data = vec![];
    rsa_data.extend(data);
    rsa_data.extend(&n[..]);
    rsa_data.extend(&e[..]);
    stream.write(&rsa_data)?;

    let mut data = [0 as u8; 1024];

    let size = stream.read(&mut data)?;

    println!("Recieved AES key, decrypting...");

    let aes_key = rsa_decrypt(&rsa_priv_key, &data[0..size]).to_digits::<u8>(Order::Msf);

    let aes = AES::from_key_and_iv(&aes_key[0..32], &aes_key[32..48]);

    println!("AES key exchange successful");

    if args.debug {
        println!("{}", aes);
    }

    loop {
        print!("  echo >> ");
        std::io::stdout().flush()?;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        buffer.pop();
        if buffer == String::from("quit") { break; }

        if args.debug {
            println!("pre_enc={:?}", buffer.as_bytes());
        }
        let message = aes.encrypt(buffer.as_bytes());
        if args.debug {
            println!("enc_msg={:?}", message);
        }
        stream.write(&message[..])?;

        let size = stream.read(&mut data)?;
        let echo_dec = aes.decrypt(&data[0..size]);
        let echo_msg = String::from_utf8_lossy(&echo_dec[..]).to_string();

        println!("Server >> {}", echo_msg);
    }

    Ok(())
}
