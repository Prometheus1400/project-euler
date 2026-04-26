use lib::{prelude::run, primes};

fn main() {
    run(0, || {
        primes::sieve_of_eratosthenes(primes::prime_number_theorem(10001)).into_iter().nth(10000).unwrap()
    });
}
