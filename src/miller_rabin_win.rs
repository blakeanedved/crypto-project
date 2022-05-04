use num_bigint::{BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::One;
use rand::prelude::ThreadRng;

fn miller_rabin_n(n: &BigUint, k: u32, rng: &mut ThreadRng) -> bool {
    let two = BigUint::from(2u8);
    let n_minus_1 = n - &BigUint::from(1u8);
    let n_minus_2 = n - &two;

    let (r, d) = {
        let mut temp = n_minus_1.clone();
        let mut r: i32 = 0;

        while temp.is_even() {
            temp /= &two;
            r += 1;
        }

        (r, temp)
    };

    for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &n_minus_2);

        let mut x = a.modpow(&d, &n);

        if x.is_one() || &x == &n_minus_1 {
            continue;
        }

        let mut flag = true;
        for _ in 0..(r - 1) {
            x = x.modpow(&two, &n);

            if &x == &n_minus_1 {
                flag = false;
                break;
            }
        }

        if flag {
            return false;
        }
    }

    return true;
}

fn gen_n(max_bits: u32, rng: &mut ThreadRng) -> BigUint {
    let mut x = rng.gen_biguint(max_bits as usize);

    while x.is_even() {
        x = rng.gen_biguint(max_bits as usize);
    }

    x
}

pub fn miller_rabin(max_bits: u32, k: u32, rng: &mut ThreadRng) -> BigUint {
    let mut n = gen_n(max_bits, rng);

    while !miller_rabin_n(&n, k, rng) {
        n = gen_n(max_bits, rng);
    }

    n
}
