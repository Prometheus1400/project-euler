use lib::prelude::run;

fn main() {
    run(5, || {
        (((1..=100).sum::<i32>()).pow(2)) - ((1..=100).map(|x: i32| x.pow(2)).sum::<i32>())
    });
}
