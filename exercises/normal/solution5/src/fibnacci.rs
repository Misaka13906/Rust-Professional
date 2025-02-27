pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    let mut f = [0; 100];
    f[1] = 1;
    let mut i = 2;
    let mut ans = 1;
    while f[i-1] < threshold && i < 100 {
        f[i] = f[i-1] + f[i-2];
        if f[i] % 2 == 1 && f[i] < threshold {
            ans += f[i];
        }
        i += 1;
    }
    return ans;
}
