#[cfg(target_family = "unix")]
use crate::miller_rabin::miller_rabin;
#[cfg(target_family = "windows")]
use crate::miller_rabin_win::miller_rabin;
#[cfg(target_family = "windows")]
use num_bigint::BigUint;
#[cfg(target_family = "unix")]
use rug::{integer::Order, Complete, Integer};

#[cfg(target_family = "unix")]
#[derive(Debug)]
pub struct RSAPublicKey {
    pub n: Integer,
    pub e: Integer,
}

#[cfg(target_family = "unix")]
#[derive(Debug)]
pub struct RSAPrivateKey {
    pub d: Integer,
    pub n: Integer,
}

#[cfg(target_family = "windows")]
#[derive(Debug)]
pub struct RSAPublicKey {
    pub n: BigUint,
    pub e: BigUint,
}

#[cfg(target_family = "windows")]
#[derive(Debug)]
pub struct RSAPrivateKey {
    pub d: BigUint,
    pub n: BigUint,
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
    #[cfg(target_family = "unix")]
    pub fn new(n: Integer, e: Integer) -> Self {
        Self { n, e }
    }

    #[cfg(target_family = "windows")]
    pub fn new(n: BigUint, e: BigUint) -> Self {
        Self { n, e }
    }

    #[cfg(target_family = "unix")]
    pub fn to_bytes(&self) -> (Vec<u8>, Vec<u8>) {
        (
            self.n.to_digits::<u8>(Order::Msf),
            self.e.to_digits::<u8>(Order::Msf),
        )
    }

    #[cfg(target_family = "windows")]
    pub fn to_bytes(&self) -> (Vec<u8>, Vec<u8>) {
        (self.n.to_bytes_be(), self.e.to_bytes_be())
    }

    #[cfg(target_family = "unix")]
    pub fn from_bytes(n: &[u8], e: &[u8]) -> Self {
        RSAPublicKey::new(
            Integer::from_digits(n, Order::Msf),
            Integer::from_digits(e, Order::Msf),
        )
    }

    #[cfg(target_family = "windows")]
    pub fn from_bytes(n: &[u8], e: &[u8]) -> Self {
        RSAPublicKey::new(BigUint::from_bytes_be(n), BigUint::from_bytes_be(e))
    }
}

impl RSAPrivateKey {
    #[cfg(target_family = "unix")]
    pub fn new(n: Integer, d: Integer) -> Self {
        Self { n, d }
    }

    #[cfg(target_family = "windows")]
    pub fn new(n: BigUint, d: BigUint) -> Self {
        Self { n, d }
    }
}

#[cfg(target_family = "unix")]
pub fn rsa_encrypt(key: &RSAPublicKey, message: &[u8]) -> Vec<u8> {
    let message = Integer::from_digits(message, Order::Msf);
    let c = match message.pow_mod(&key.e, &key.n) {
        Ok(c) => c,
        Err(_) => {
            println!("Error occured with\nE={}\nN={}", &key.e, &key.n);
            panic!();
        }
    };
    c.to_digits::<u8>(Order::Msf)
}

#[cfg(target_family = "unix")]
pub fn rsa_decrypt(key: &RSAPrivateKey, message: &[u8]) -> Vec<u8> {
    let message = Integer::from_digits(message, Order::Msf);
    let m = match message.pow_mod(&key.d, &key.n) {
        Ok(m) => m,
        Err(_) => unreachable!(),
    };
    m.to_digits::<u8>(Order::Msf)
}

#[cfg(target_family = "windows")]
pub fn rsa_encrypt(key: &RSAPublicKey, message: &[u8]) -> Vec<u8> {
    BigUint::from_bytes_be(message)
        .modpow(&key.e, &key.n)
        .to_bytes_be()
}

#[cfg(target_family = "windows")]
pub fn rsa_decrypt(key: &RSAPrivateKey, message: &[u8]) -> Vec<u8> {
    BigUint::from_bytes_be(message)
        .modpow(&key.d, &key.n)
        .to_bytes_be()
}

#[cfg(target_family = "unix")]
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

#[cfg(target_family = "windows")]
pub fn rsa_key_gen() -> (RSAPublicKey, RSAPrivateKey) {
    use std::str::FromStr;

    use num_bigint_dig::ModInverse;
    use num_integer::Integer;

    let mut rng = rand::thread_rng();

    let (p, q) = if std::path::Path::new(".rsa_key").exists() {
        println!("RSA Key found, loading...");

        let nums = std::fs::read_to_string(".rsa_key")
            .unwrap()
            .split("\n")
            .map(|s| BigUint::from_str(&s).unwrap())
            .collect::<Vec<_>>();

        if nums.len() != 2 {
            panic!()
        }

        (nums[0].clone(), nums[1].clone())
    } else {
        let (p, q) = (
            miller_rabin(1024, 7, &mut rng),
            miller_rabin(1024, 7, &mut rng),
        );

        println!("Saving RSA Key");

        std::fs::write(
            ".rsa_key",
            vec![p.clone(), q.clone()]
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();

        (p, q)
    };

    let n = p.clone() * &q;
    let phi = BigUint::from((p - 1u32).lcm(&(q - 1u32)));
    let e = BigUint::from(65537u32);
    let d = e.clone().mod_inverse(phi).unwrap().to_biguint().unwrap();

    (RSAPublicKey::new(n.clone(), e), RSAPrivateKey::new(n, d))
}
