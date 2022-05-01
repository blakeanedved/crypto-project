use rug::{Complete, Integer};
use rug::integer::Order;

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
        Err(_) => unreachable!(),
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
    let p = Integer::from(11i32);
    let q = Integer::from(17i32);

    let mut n = Integer::new();
    (&p * &q).complete_into(&mut n);
    let phi = (p - 1u32).lcm(&(q - 1u32));
    let e = Integer::from(67i32);
    let d = match e.clone().pow_mod(&Integer::from(-1i32), &phi) {
        Ok(d) => d,
        Err(_) => unreachable!(),
    };

    (RSAPublicKey::new(n.clone(), e), RSAPrivateKey::new(n, d))
}
