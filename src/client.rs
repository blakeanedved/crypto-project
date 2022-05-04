use std::io::{Read, Write};
use std::net::TcpStream;

use crate::rsa::{rsa_decrypt, rsa_key_gen};
use crate::{aes::AES, Args, Commands};

pub fn client(args: Args) -> anyhow::Result<()> {
    let host = match args.command {
        Commands::Client { host } => host,
        _ => unreachable!(),
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

    let (n, e) = rsa_pub_key.to_bytes();

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

    let aes_key = rsa_decrypt(&rsa_priv_key, &data[0..size]);

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

        if cfg!(target_os = "windows") {
            buffer.pop();
        }

        if buffer == String::from("quit") {
            break;
        }

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
