use rug::{Complete, Integer};
use rug::integer::Order;

use crate::miller_rabin::miller_rabin;

#[derive(Debug)]
pub struct RSAPublicKey {
    pub n: Integer,
    pub e: Integer,
}

#[derive(Debug)]
pub struct RSAPrivateKey {
    pub d: Integer,
    pub n: Integer,
}

impl std::fmt::Display for RSAPublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RSAPublicKey {{\n\tn: {},\n\te: {}\n}}", self.n, self.e)
    }
}

impl std::fmt::Display for RSAPrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "RSAPrivateKey {{\n\tn: {},\n\td: {}\n}}", self.n, self.d)
    }
}

impl RSAPublicKey {
    pub fn new(n: Integer, e: Integer) -> Self {
        Self { n, e }
    }
}

impl RSAPrivateKey {
    pub fn new(n: Integer, d: Integer) -> Self {
        Self { n, d }
    }
}

pub fn rsa_encrypt(key: &RSAPublicKey, message: Integer) -> Integer {
    let c = match message.pow_mod(&key.e, &key.n) {
        Ok(c) => c,
        Err(_) => {
            println!("Error occured with\nE={}\nN={}", &key.e, &key.n);
            panic!();
        }
    };
    c
}

pub fn rsa_decrypt(key: &RSAPrivateKey, message: &[u8]) -> Integer {
    let message = Integer::from_digits(message, Order::Msf);
    let m = match message.pow_mod(&key.d, &key.n) {
        Ok(m) => m,
        Err(_) => unreachable!(),
    };
    m
}

pub fn rsa_key_gen() -> (RSAPublicKey, RSAPrivateKey) {
    let mut rng = rug::rand::RandState::new();
    let p = miller_rabin(2048, 7, &mut rng);
    let q = miller_rabin(2048, 7, &mut rng);

    let mut n = Integer::new();
    (&p * &q).complete_into(&mut n);
    let phi = (p - 1u32).lcm(&(q - 1u32));
    let e = Integer::from(65537i32);
    let d = match e.clone().pow_mod(&Integer::from(-1i32), &phi) {
        Ok(d) => d,
        Err(_) => unreachable!(),
    };

    (RSAPublicKey::new(n.clone(), e), RSAPrivateKey::new(n, d))
}
