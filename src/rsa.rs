use std::ptr::NonNull;

use crypto_bigint::{Checked, NonZero, Wrapping, U1024};

struct RSAPubKey {
    n: U1024,
    e: i32,
}

fn gcd(a: NonZero<U1024>, b: NonZero<U1024>) -> NonZero<U1024> {
    let mut k = 0;
    let zero = U1024::ZERO;
    let (a, b) = if a > b { (a, b) } else { (b, a) };
    let mut q: U1024 = U1024::from(1u32);
    let mut r0 = a;
    let mut r1 = b;
    while q != zero {
        q = r0.wrapping_rem(&r1);
        r0 = r1;
        // r1 = q;
    }
    r1
}

fn lcm(a: NonZero<U1024>, b: NonZero<U1024>) -> NonZero<U1024> {
    NonZero::new((a.saturating_mul(&b)) / gcd(a, b)).unwrap()
}
fn rsa_encrypt(key: RSAPubKey, message: i64) -> i64 {
    // message.pow(key.e as u32) % key.n
    0
}

fn rsa_key_gen() -> (RSAPubKey, i64) {
    let p = Wrapping(U1024::from(11u32));
    let q = Wrapping(U1024::from(17u32));

    let one = Wrapping(U1024::ONE);

    let n = p * q;
    let phi = (p - one) * (q - one);
    let e = 3;
    // let d = (e ^ -1) % phi;

    (
        RSAPubKey {
            n: U1024::from(1u32),
            e,
        },
        1,
    )
}
