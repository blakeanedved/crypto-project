use std::{ops::Sub, cmp::Ordering};

use rug::{Integer, ops::{Pow, SubFrom}};
mod constants;

use constants::*;

macro_rules! check_non_trivial_divisor {
    ($num:ident, $n:ident) => {
        let gcd = Integer::from($num.gcd_ref(&$n));
        if gcd != 1i32 && gcd != $n {
            return Composite;
        }
    }
}

macro_rules! get_random {
    ($high:expr, $rng:ident) => {{
        let mut x = Integer::random_below(Integer::from(&$high - &2i32), &mut $rng);
        while x < 2i32 {
            x = Integer::random_below(Integer::from(&$high - &2i32), &mut $rng);
        }
        x
    }}
}

macro_rules! split {
    ($v:ident) => {{
        let s = $v.find_one(1).unwrap();
        let (q, _) = $v.div_rem_floor(Integer::from(s.pow(2)));
        (q, s)
    }}
}

#[derive(Debug, PartialEq)]
pub enum Primality {
    Composite, ProbablyPrime, Prime
}

use Primality::*;

static TWOPOW31: i32 = 2i32.pow(31);

fn steps_1_2(n: &Integer) -> Primality {
    // step 2
    if n.is_perfect_square() {
        return Composite;
    }

    // step 1
    if n <= &TWOPOW31 {
        let sqrt = Integer::from(n.sqrt_ref());

        let mut i = 1;
        while i < PRIME_LIST_LEN && PRIME_LIST[i] <= sqrt {
            if n.is_divisible_u(PRIME_LIST[i]) {
                return Composite;
            }
            i += 1;
        }

        if sqrt < B {
            return Prime;
        }

    } else {
        let mut i = 1;
        while i < PRIME_LIST_LEN {
            if n.is_divisible_u(PRIME_LIST[i]) {
                return Composite;
            }

            i += 1;
        }
    }

    ProbablyPrime
}

fn power_of_x(exp: &Integer, n: &Integer, b: &Integer, c: &Integer) -> (Integer, Integer) {
    (Integer::new(), Integer::new())
}

fn square_mod(poly: (&Integer, &Integer), n: &Integer, b: &Integer, c: &Integer) -> (Integer, Integer) {
    (Integer::new(), Integer::new())
}

fn mult_x_mod(poly: (&Integer, &Integer), n: &Integer, b: &Integer, c: &Integer) -> (Integer, Integer) {
    (Integer::new(), Integer::new())
}

fn steps_3_4_5(n: &Integer, b: &Integer, c: &Integer) -> Primality {
    let n_is_1_mod_4 = Integer::from(n % 4i32) == 1i32;
    let temp = if n_is_1_mod_4 {
        Integer::from(n - 1i32)
    } else {
        Integer::from(n + 1i32)
    };

    let (r, s) = split!(temp);
    let t = Integer::from((s - 1) / 2);

    let x_t = power_of_x(&t, n, b, c);

    let mut temp_poly = square_mod((&x_t.0, &x_t.1), n, b, c);

    temp_poly = mult_x_mod((&temp_poly.0, &temp_poly.1), n, b, c);
    
    let mut i = Integer::from(1i32);
    while &i < &r {
        temp_poly = square_mod((&temp_poly.0, &temp_poly.1), n, b, c);
        i += 1;
    }

    if n_is_1_mod_4 {
        temp_poly = mult_x_mod((&temp_poly.0, &temp_poly.1), n, b, c);
    }

    if temp_poly.0.cmp0() != Ordering::Equal {
        return Composite;
    }

    ProbablyPrime
}

pub fn rqft(n: Integer, rounds: u32) -> Primality {
    let mut rng = rug::rand::RandState::new();
    let mut j_bb4c = 0;
    let mut b = Integer::new();
    let mut c = Integer::from(-1i32);
    let mut bb4c;

    // check if n is even for the first condition and return prime or composite if it is 2
    if n.is_even() {
        if n == 2i32 {
            return Prime;
        } else {
            return Composite;
        }
    }

    // we dont want to process negative numbers or 0/1
    if n <= 1i32 {
        return Composite;
    }

    let mut result = steps_1_2(&n);

    if result != ProbablyPrime {
        return result;
    }

    for _round in 0..rounds {
        // construct c such that -c is a square so we dont have to compute jacobi(-c/n)
        while &c < &0i32 {
            c.pow_mod_mut(&TWO, &n).unwrap();

            c.sub_from(&n);
        }

        check_non_trivial_divisor!(c, n);

        for _ in 0..B {
            b = get_random!(n, rng);
            bb4c = Integer::from((&b).pow(2));
            bb4c += 4 * &c;
            j_bb4c = bb4c.jacobi(&n);
            if j_bb4c == -1 {
                check_non_trivial_divisor!(bb4c, n);
                check_non_trivial_divisor!(b, n);
                break;
            }
        }

        if j_bb4c != -1 {
            eprintln!("could not find a pair (b,c) assuming n is prime");
            return Prime;
        } else {
            result = steps_3_4_5(&n, &b, &c);
            if result != ProbablyPrime {
                return result;
            }
        }
    }

    ProbablyPrime
}
