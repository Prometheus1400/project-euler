use std::{fmt::Display, time::Instant};

pub fn banner(id: u32) {
    let txt = format!("Project Euler #{id:04}");
    let line = "═".repeat(txt.len() + 2);
    println!("╔{line}╗\n║ {txt} ║\n╚{line}╝");
}

pub fn answer<T: Display>(ans: T) {
    println!("└─ Answer: {ans}");
}

pub fn elapsed(start: Instant) {
    println!("└─ Elapsed: {:?}", start.elapsed());
}

pub fn run<T: Display>(id: u32, solve: impl FnOnce() -> T) {
    banner(id);
    let start = Instant::now();
    let ans = solve();
    answer(ans);
    elapsed(start);
}
