use std::ptr::NonNull;

use crypto_bigint::{Checked, NonZero, Wrapping, U1024, U128};

pub struct RSAPubKey {
    n: U1024,
    e: U1024,
}

fn to_nonzero(n: U1024) -> NonZero<U1024> {
    NonZero::new(n).unwrap()
}

fn gcd(a: NonZero<U1024>, b: NonZero<U1024>) -> NonZero<U1024> {
    let zero = U1024::ZERO;
    let (a, b) = if a > b { (a, b) } else { (b, a) };
    let mut q: U1024 = U1024::from(1u32);
    let mut r0 = a;
    let mut r1 = b;
    loop {
        // println!("q={}, r0={}, r1={}", q, r0, r1);
        q = r0.wrapping_rem(&r1);
        if q <= zero {
            return r1;
        }
        r0 = r1;
        r1 = NonZero::new(q).unwrap();
    }
}

pub fn lcm(a: NonZero<U1024>, b: NonZero<U1024>) -> NonZero<U1024> {
    println!("a={} b={} a*b={}", a, b, a.saturating_mul(&b));
    NonZero::new((a.saturating_mul(&b)) / gcd(a, b)).unwrap()
}
pub fn rsa_encrypt(key: RSAPubKey, message: U128) -> U1024 {
    U1024::ZERO
}

pub fn rsa_decrypt(key: U1024, message: U1024) -> U128 {
    U128::ZERO
}

pub fn rsa_key_gen() -> (RSAPubKey, U1024) {
    let p = U1024::from(11u32);
    let q = U1024::from(17u32);

    let one = U1024::ONE;

    let n = p.wrapping_mul(&q);
    let phi = lcm(
        to_nonzero(p.wrapping_sub(&one)),
        to_nonzero(q.wrapping_sub(&one)),
    );
    let e = U1024::from(65536u32);
    let d = (one.wrapping_div(&e)).wrapping_rem(&phi);

    (
        RSAPubKey {
            n: U1024::from(1u32),
            e,
        },
        d,
    )
}
