const MAX: usize = 100001;
pub fn goldbach_conjecture() -> String {
    let mut is_prime = [true; MAX];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..MAX {
        if is_prime[i] {
            for j in 2..MAX {
                if i*j >= MAX { break; }
                is_prime[i*j] = false;
            }
        }
    }

    let mut ans = Vec::new();
    let mut cnt = 0;
    for x in 3..MAX {
        if x % 2 == 0 || is_prime[x] || satisfy(x, &is_prime) {
            continue;
        }
        println!("{x}");
        ans.push(x);
        cnt += 1;
        if cnt >= 2 {
            return format!("{},{}", ans[0], ans[1]);
        }
    }
    if cnt < 2 {
        panic!("value of MAX is not enough")
    } else {
        return format!("{},{}", ans[0], ans[1]);
    }
}

fn satisfy(x: usize, is_prime: &[bool; MAX]) -> bool {
    let max = std::primitive::f32::sqrt(((x-3)/2) as f32) as usize;
    for i in 1..=max {
        if is_prime[x - 2*i*i] {
            return true;
        }
    }
    return false;
}