use crate::rsa;
use rug::Integer;

pub fn test() {
    let (public, private) = rsa::rsa_key_gen();
    println!("public={:?} private={:?}", public, private);

    let c = rsa::rsa_encrypt(&public, Integer::from(4u32));
    println!("{}", c);

    let m = rsa::rsa_decrypt(&private, c);

    println!("{}", m);
}
