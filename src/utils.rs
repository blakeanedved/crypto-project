use crate::aes;
use crate::rsa;
use rug::Integer;

pub fn test() {
    let x = aes::AES::new();
    // x.key_expansion();
    println!("{:x?}", x.key_expansion())
}
