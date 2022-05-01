use rug::Integer;

fn miller_rabin_n(n: &Integer, k: u32, rng: &mut rug::rand::RandState) -> bool {
    let n_minus_1 = Integer::from(n - &1i32);
    let n_minus_3 = Integer::from(n - &2i32);
    let two = Integer::from(2i32);

    let (r, d) = {
        let mut temp = n.clone() - 1i32;
        let mut r: i32 = 0;

        while temp.is_even() {
            temp.div_exact_u_mut(2);
            r += 1;
        }

        (r, temp)
    };


    for _ in 0..k {
        let mut a = Integer::from(n_minus_3.random_below_ref(rng));
        a += 2i32;

        let mut x = a.pow_mod(&d, n).unwrap();

        if &x == &1i32 || &x == &n_minus_1 {
            continue;
        }

        let mut flag = true;
        for _ in 0..(r - 1) {
            x.pow_mod_mut(&two, n).unwrap();

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

fn gen_n(max_bits: u32, rng: &mut rug::rand::RandState) -> Integer {
    let mut x = Integer::from(Integer::random_bits(max_bits, rng));

    while x.is_even() {
        x = Integer::from(Integer::random_bits(max_bits, rng));
    }

    x
}

pub fn miller_rabin(max_bits: u32, k: u32, rng: &mut rug::rand::RandState) -> Integer {
    let mut n = gen_n(max_bits, rng);

    while !miller_rabin_n(&n, k, rng) {
        n = gen_n(max_bits, rng);
    }

    n
}
