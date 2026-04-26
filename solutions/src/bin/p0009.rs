use lib::prelude::run;

fn is_pythagorean_triplet(a: usize, b: usize, c: usize) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2)
}

fn main() {
    run(0, || {
        for a in 1..=1000 {
            let b = (1000 * (500 - a)) / (1000 - a);
            let c = 1000 - a - b;
            if is_pythagorean_triplet(a, b, c) {
                return a * b * c;
            }
        }
        panic!();
    });
}
