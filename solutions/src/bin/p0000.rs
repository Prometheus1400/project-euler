use lib::prelude::run;

fn main() {
    run(0, || {
        let n: usize = 827_000;
        let ans: usize = (1..=n).step_by(2).map(|n| n.pow(2)).sum();
        ans
    });
}
