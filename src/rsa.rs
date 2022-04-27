use rug::{Complete, Integer};

pub struct RSAPublicKey {
    n: Integer,
    e: Integer,
}

pub struct RSAPrivateKey {
    d: Integer,
    n: Integer,
}

pub fn rsa_encrypt(key: RSAPublicKey, message: Integer) -> Integer {
    let c = match message.pow_mod(&key.e, &key.n) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };
    c
}

pub fn rsa_decrypt(key: RSAPrivateKey, message: Integer) -> Integer {
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
    let e = Integer::from(65537i32);
    let mut d = Integer::new();
    (1u32 / &e).complete_into(&mut d);
    d = d % phi;

    (RSAPublicKey { n: n.clone(), e }, RSAPrivateKey { d, n })
}
