use rand::{thread_rng, Rng};

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub as u128, a, p)
}

// https://en.wikipedia.org/wiki/Modular_exponentiation#Right-to-left_binary_method
fn mod_pow(mut base: u128, mut exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let m = modulus as u128;
    base %= m;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % m;
        }
        base = (base * base) % m;
        exponent >>= 1;
    }
    result as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_pow() {
        for (b, e, m) in [(4, 13, 497), (5, 3, 13), (23, 5, 6)] {
            assert_eq!(u64::pow(b, e) % m, mod_pow(b as u128, e as u64, m));
        }
    }
}
