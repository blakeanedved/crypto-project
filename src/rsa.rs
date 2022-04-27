use rug::Integer;

pub struct RSAPubKey {
    n: Integer,
    e: Integer,
}

pub struct RSAPrivateKey {
    d: Integer,
    e: Integer,
}

pub fn rsa_encrypt(key: RSAPubKey, message: Integer) -> Integer {
    let c = match message.pow_mod(&key.e, &key.n) {
        Ok(c) => c,
        Err(_) => unreachable!(),
    };
    c
}

pub fn rsa_decrypt(key: RSAPrivateKey, message: Integer) -> Integer {
    let m = match message.pow_mod(&key.d, &key.e) {
        Ok(m) => m,
        Err(_) => unreachable!(),
    };
    m
}

pub fn rsa_key_gen() -> (RSAPubKey, Integer) {
    let p = Integer::from(11i32);
    let q = Integer::from(17i32);

    let n = p * q;
    let phi = (p - 1u32).lcm(&(q - 1u32));
    let e = Integer::from(65537i32);
    let d = (1 / e) % phi;

    (RSAPubKey { n, e }, d)
}
