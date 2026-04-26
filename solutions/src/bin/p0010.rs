use lib::{prelude::run, primes};

fn main() {
    run(0, || {
        primes::sieve_of_eratosthenes(2_000_000).iter().sum::<usize>()
    });
}
