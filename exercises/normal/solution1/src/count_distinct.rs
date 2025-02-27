pub fn new_count_distinct(input_str: &str) -> usize {
    const MOD:u64 = 100003;
    let arr:Vec<&str> = input_str.split(',').collect();
    let mut cnt = [false; MOD as usize];
    let mut ans = 0;
    for s in arr {
        let mut hash_res: u64 = 0;
        for ch in s.as_bytes() {
            hash_res = (hash_res*128 + (*ch) as u64) % MOD;
        }
        cnt[hash_res as usize] = true;
    }
    for f in cnt {
        if f { ans += 1; }
    }
    ans
}
