use itertools::Itertools;
use lib::prelude::run;

fn main() {
    run(1, || {
        let n = 1000;
        let ans: usize = (3..n).step_by(3).chain((5..n).step_by(5)).unique().sum();
        ans
    });
}
