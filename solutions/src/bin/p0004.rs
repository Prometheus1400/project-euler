#![allow(unused)]

use std::{collections::HashMap, vec};

use lib::{
    prelude::run,
    primes,
};

fn main() {
    run(5, || {
        let mut prime_factorizations = vec![];
        for n in 1..=20 {
            prime_factorizations.push(primes::prime_factorization_with_count(n));
        }

        let mut prime_counts: HashMap<usize, usize> = HashMap::new();
        for prime_factorization in prime_factorizations.into_iter() {
            for (p, count) in prime_factorization {
                prime_counts.insert(p, *prime_counts.get(&p).unwrap_or(&0).max(&count));
            }
        }

        let ans: usize = prime_counts
            .into_iter()
            .map(|(p, count)| p.pow(count as u32))
            .product();
        ans
    });
}
