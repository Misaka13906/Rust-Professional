pub fn find_max_prime_factor(number: u128) -> u128 {
    const MAX: usize = 20000000000; //2e10
    let mut x = number;
    0
}

// miller rabin
fn is_prime(x: u128) -> bool {
    if x == 2 || x == 3 { return true; }
    if x % 2 == 0 || x < 2 { return false; }
    let base: [u128; 7] = [2,325,9375,28178,450775,9780504,1795265022];
    let mut pw = 1;
    while (x-1) % (pw*2) == 0 { pw *= 2; }
    for a in base {
        let mut res: Vec<u128> = Vec::new();
        let mut u = (x-1) / pw;
        let mut r = qpow(a, u, x);
        while u < x {
            if r == 0 {break;}
            res.push(r);
            r = r*r%x;
            u *= 2;
        }
        if !res.is_empty() && *res.last().unwrap() != 1u128 {
            return false;
        }
        for &r in res.iter().rev() {
            if r == x-1 {
                break;
            }
            if r != 1u128 && r != x-1 {
                return false;
            }
        }
    }
    true
}

fn qpow(a: u128, n: u128, p: u128) -> u128 {
    let mut res: u128 = 1u128;
    let mut base = a;
    let mut n = n;
    while n > 0 {
        if n & 1 == 1 {
            res = res * base % p;
        }
        base = base * base % p;
        n >>= 1;
    }
    res
}
