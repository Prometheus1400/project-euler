use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    vec,
};

/// Generate all prime numbers less than or equal to `n`
/// using the Sieve of Eratosthenes.
///
/// Returns an empty vector for `n < 2`.
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    match n {
        0 | 1 => vec![],
        2 => vec![2],
        _ => {
            let mut sieve = vec![true; n + 1];
            sieve[0] = false;
            sieve[1] = false;

            let mut p: usize = 2;
            while p.pow(2) <= n {
                if sieve[p] {
                    for n in (p.pow(2)..=n).step_by(p) {
                        sieve[n] = false;
                    }
                }
                p += 1;
            }

            sieve
                .into_iter()
                .enumerate()
                .skip(2)
                .filter(|(_, v)| *v)
                .map(|(i, _)| i)
                .collect()
        }
    }
}

/// Get an upper bound B where B contains at least N prime numbers
/// for n >= 688383 a good bound is U = n(ln(n) + ln(ln(n)))
/// for n < 688383 estimate U = n * 20
pub fn prime_number_theorem(n: usize) -> usize {
    if n < 688383 {
        n * 20
    } else {
        let nf: f64 = n as f64;
        (nf * (nf.ln() + nf.ln().ln()).ceil()) as usize
    }
}

/// Compute the unique prime factors of `n`
/// using repeated division by primes up to `sqrt(n)`.
///
/// Returns distinct factors only (no multiplicities).
pub fn prime_factorization(mut n: usize) -> HashSet<usize> {
    let primes = sieve_of_eratosthenes(n.isqrt().add(1));
    let mut prime_factors = HashSet::new();
    let mut factored = true;
    while factored {
        factored = false;
        for p in primes.iter() {
            if n.is_multiple_of(*p) {
                prime_factors.insert(*p);
                n /= p;
                factored = true;
                break;
            }
        }
    }

    if n > 1 {
        prime_factors.insert(n);
    }

    prime_factors
}

/// Compute the unique prime factors of `n`
/// using repeated division by primes up to `sqrt(n)`.
///
/// Returns factors and their counts
pub fn prime_factorization_with_count(mut n: usize) -> HashMap<usize, usize> {
    let primes = sieve_of_eratosthenes(n.isqrt().add(1));
    let mut prime_factors = HashMap::new();
    let mut factored = true;
    while factored {
        factored = false;
        for p in primes.iter() {
            if n.is_multiple_of(*p) {
                prime_factors.insert(*p, prime_factors.get(p).unwrap_or(&0) + 1);
                n /= p;
                factored = true;
                break;
            }
        }
    }

    if n > 1 {
        prime_factors.insert(n, 1);
    }

    prime_factors
}
