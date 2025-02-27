pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let _ = num_str.to_lowercase();
    let mut num:u32 = 0;
    let mut base:u32 = 0;
    for ch in num_str.chars() {
        if ch == ')' {
            break;
        }
        if ch == '(' {
            base = 0;
        } else {
            base = base * 10 + (ch as u32 ^ 48);
        }
    }
    for ch in num_str.chars() {
        if ch == '(' {
            break;
        }
        num = num * base + if ch >= '0' && ch <= '9' {
            ch as u32 ^ 48
        } else if ch >= 'a' && ch <= 'z' {
            ch as u32 - 97 + 10
        } else {
            assert!(false); 0 
        };
    }
    let mut ans = vec![];
    while num > 0 {
        ans.push(if num % to_base < 10 {
            ((num % to_base) ^ 48) as u8 as char
        } else {
            ((num % to_base) - 10 + 97) as u8 as char
        });
        num /= to_base;
    }
    ans.reverse();
    return String::from_iter(ans);
}
