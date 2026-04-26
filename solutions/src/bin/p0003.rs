use lib::prelude::run;
use lib::primes::prime_factorization;

fn main() {
    run(3, || {
        let ans = prime_factorization(600_851_475_143)
            .into_iter()
            .max()
            .unwrap();
        ans
    });
}
