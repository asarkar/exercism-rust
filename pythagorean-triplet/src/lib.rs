use std::collections::HashSet;

/*
 * We use Euclid's formula of generating a tuple.
 * https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple
 *
 * a = m^2 - n^2, b = 2mn, c = m^2 + n^2, where m > n > 0 ---(i)
 * a + b + c = P ---(ii)
 *
 * Combining equations (i) and (ii), we have:
 * 2m^2 + 2mn = P ---(iii)
 *
 * Since m > n > 0, 1 <= n <= m - 1.
 * Putting n=1 in equation (iii), we have:
 * 2m^2 + 2m - P = 0, ax^2 + bx + c = 0, a=2, b=2, c=-P
 *  m = (-b +- sqrt(b^2 - 4ac)) / 2a
 *  => (-2 +- sqrt(4 + 8P)) / 4
 *  => (-1 +- sqrt(1 + 2P)) / 2
 *
 * Since m > 0, sqrt(b^2 - 4ac) > -b, the only solution is
 *  (-1 + sqrt(1 + 2P)) / 2 ---(iv)
 *
 * Putting n=m-1 in equation (iii), we have:
 * 2m^2 + 2m(m - 1) - P = 0
 *  => 4m^2 - 2m - P = 0, ax^2 + bx + c = 0, a=4, b=-2, c=-P
 *  m = (-b +- sqrt(b^2 - 4ac)) / 2a
 *  => (2 +- sqrt(4 + 16P)) / 8
 *  => (1 +- sqrt(1 + 4P)) / 4
 *
 * Since m > 0, the only solution is (1 + sqrt(1 + 4P)) / 4 ---(v)
 *
 * From equation (iii), m^2 + mn = P/2; since P/2 is constant,
 * when n is the smallest, m must be the largest, and vice versa.
 *
 * Thus, (1 + sqrt(1 + 4P)) / 4 <= m <= (-1 + sqrt(1 + 2P)) / 2 ---(vi)
 *
 * Solving equation (iii) for n, we have:
 *  n = (P - 2m^2) / 2m ---(vii)
 *
 * We iterate for m within the bounds given by the inequality (vi)
 * and check when the corresponding n given by equation (vii) is
 * an integer.
 *
 * Despite generating all primitive triples, Euclid's formula does not
 * produce all triples - for example, (9, 12, 15) cannot be generated using
 * integer m and n. This can be remedied by inserting an additional
 * parameter k to the formula. The following will generate all Pythagorean
 * triples uniquely.
 * a = k(m^2 - n^2), b = 2kmn, c = k(m^2 + n^2), for k >= 1.
 *
 * Thus, we iterate for integer values of P/k until P < 12,
 * lowest possible perimeter corresponding to the triple (3, 4, 5).
 */
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for k in 1..=(sum / 12) {
        let sum = (sum as f64) / (k as f64);
        if sum.fract() != 0.0 {
            continue;
        }

        let min_m = ((1.0 + (1.0 + 4.0 * sum).sqrt()) / 4.0) as usize;
        let max_m = ((-1.0 + (1.0 + 2.0 * sum).sqrt()) / 2.0) as usize;

        for m in min_m..=max_m {
            let x = (2 * m * m) as f64;
            let numerator = sum - x;
            if numerator <= 0.0 {
                continue;
            }
            let n = numerator / (2 * m) as f64;

            if (m as f64) > n && n.fract() == 0.0 {
                let m = m as u32;
                let n = n as u32;
                let a = k * (m * m - n * n);
                let b = k * (2 * m * n);
                let c = k * (m * m + n * n);
                let mut t = [a, b, c];
                t.sort_unstable();
                triplets.insert(t);
            }
        }
    }

    triplets
}
