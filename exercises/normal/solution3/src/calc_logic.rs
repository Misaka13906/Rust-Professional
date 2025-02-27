pub fn new_birthday_probability(n: u32) -> f64 {
    let mut res = 1.0;
    for i in ((365-n+1)..365).rev() {
        res *= i as f64 / 365.0;
    }
    return 1.0 - res;
}
