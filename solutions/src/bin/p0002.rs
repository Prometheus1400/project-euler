use lib::prelude::run;

fn main() {
    run(2, || {
        let mut p1 = 2; // represents previous value n
        let mut n = 3; // current value
        let mut ans = 2;

        while n <= 4_000_000 {
            if n % 2 == 0 {
                ans += n;
            }
            let next_n = p1 + n;
            p1 = n;
            n = next_n;
        }

        ans
    });
}
