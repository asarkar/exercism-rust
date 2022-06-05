#[derive(Debug)]
struct Prime {
    values: Vec<u64>,
}

impl Prime {
    fn new() -> Self {
        Self { values: vec![1] }
    }

    /*
     * A good way to speed up this method is to pre-compute and store a list of
     * all primes up to a certain bound, such as all primes up to 200.
     * Such a list can be computed with the Sieve of Eratosthenes.
     * But it's only useful if the prime generator is reused, which, for the tests,
     * isn't the case, so, we don't bother.
     */
    fn is_prime(&self, n: u64) -> bool {
        /* Suppose xy = n = √n√n. If x ≥ √n, then y ≤ √n and vice-versa.
         * Thus, if xy = n, then one of x or y must be less than or equal to √n.
         * This means that if n can be factored, one of the factors must be less
         * than or equal to √n, so, we only need to check till √n.
         */
        let x = ((n as f64).sqrt()) as u64;
        let composite = self
            .values
            .iter()
            .skip(1)
            .take_while(|&p| p <= &x)
            .any(|&p| n % p == 0);

        !composite
    }
}

impl Iterator for Prime {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut nxt = self.values[self.values.len() - 1] + 1;
        while !self.is_prime(nxt) {
            nxt += 1;
        }
        self.values.push(nxt);
        Some(nxt)
    }
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut x = n;
    let mut factors = Vec::new();

    for i in Prime::new() {
        while x > 1 && x % i == 0 {
            factors.push(i);
            x /= i;
        }
        if x <= 1 {
            break;
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let p = Prime::new();
        assert!(p.is_prime(2));
    }

    #[test]
    fn test_prime_iterator() {
        let mut p = Prime::new();
        assert_eq!(Some(2), p.next());
        assert_eq!(Some(3), p.next());
        assert_eq!(Some(5), p.next());
    }
}
