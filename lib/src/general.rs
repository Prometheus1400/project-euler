pub fn dig_vec(mut n: usize) -> Vec<usize> {
    let mut res = vec![];
    while n > 10 {
        res.push(n % 10);
        n /= 10;
    }
    if n > 0 {
        res.push(n);
    }

    res
}
