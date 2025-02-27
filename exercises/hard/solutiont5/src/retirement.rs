pub fn retire_time(time: &str, tp: &str) -> String {
    let kinds = vec!["男职工", "原法定退休年龄55周岁女职工", "原法定退休年龄50周岁女职工"];
    let mut found = false;
    let mut kind: usize = 0;
    for (i, s) in kinds.iter().enumerate() {
        if *s == tp {
            kind = i;
            found = true;
        }
    }
    if !found { panic!("人员类型字符串错误：{}", tp); }

    let birth_year = String::from(time)[0..4].trim().parse::<i32>().ok().unwrap();
    let birth_month = String::from(time)[5..7].trim().parse::<i32>().ok().unwrap();
    let former_age = vec![60, 55, 50];
    let start_year = 2025;
    if birth_year + former_age[kind] < start_year {
        return format!("{}-{:02},{:.2},0", birth_year + former_age[kind], birth_month, former_age[kind]);
    }

    let delay_rate = vec![4, 4, 2]; // 每 x 个月延迟 1 个月
    let max_age = vec![63, 58, 55];

    let late_months = (birth_year - (start_year - former_age[kind])) * 12 + birth_month;
    let mut delay_months = (late_months + delay_rate[kind] - 1) / delay_rate[kind];
    let mut retire_year = start_year + (late_months + delay_months) / 12;
    let mut retire_month = (late_months + delay_months - 1) % 12 + 1;
    
    if delay_months > (max_age[kind] - former_age[kind]) * 12 {
        delay_months = (max_age[kind] - former_age[kind]) * 12;
        retire_year = birth_year + max_age[kind];
        retire_month = birth_month;
    }

    let retire_age = former_age[kind] as f64 + delay_months as f64 / 12.0;

    if retire_age - (retire_age as i32 as f64) < 0.001 {
        format!("{}-{:02},{},{}", retire_year, retire_month, retire_age as i32, delay_months)
    } else {
        format!("{}-{:02},{:.2},{}", retire_year, retire_month, retire_age, delay_months)
    }
}
