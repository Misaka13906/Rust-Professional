pub fn dp_rec_mc(amount: u32) -> u32 {
    let w = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut f = [10000; 10000];
    let m = amount as usize;
    f[0] = 0;
    for x in w {
        f[x] = 1;
    }
    for x in w {
        for i in x..=m {
            f[i] = std::cmp::min(f[i], f[i-x] + 1);
        }
    }
    return f[m];
}
