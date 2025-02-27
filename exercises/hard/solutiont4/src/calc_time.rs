const start_date: (i32, i32, i32) = (1582, 10, 15);
const start_weekday: i32 = 5;
macro_rules! is_leap_year {
    ($year:expr) => { $year % 400 == 0 || ($year % 100 != 0 && $year % 4 == 0) };
}

pub fn time_info(time: &str) -> String {
    let nums: Vec<i32> = time.split('-').filter_map(|part| part.trim().parse::<i32>().ok()).collect();
    let [year, month, day] = nums[0..3] else { panic!("parse expression error"); };
    let in_year_days_now = get_year_days(year, month, day);
    let remain = if is_leap_year!(year) {366} else {365} - in_year_days_now;
    let offset_days = (7 + get_days_mol7((year, 1, 1)) - get_days_mol7(start_date)) % 7;
    let weekday_first = (offset_days + start_weekday - 1) % 7 +1; // 当年第一天是周几
    let weekday_now = (in_year_days_now - 1 + weekday_first - 1) % 7 +1; // 输入时间是周几
    let mut week_idx = (in_year_days_now - (8 - weekday_first) + 6) / 7 + if weekday_first <= 4 {1} else {0};
    // print!("{time}\tweekday_now: {weekday_now}\t");
    if remain < 7 && weekday_now < 4 {
        week_idx = 1;
    }
    let mut festival_in_year_days = get_festival_in_year_days(year) - in_year_days_now - 1;
    if festival_in_year_days < 0 {
        festival_in_year_days = get_festival_in_year_days(year+1) + remain - 1;
    }
    format!("{week_idx},{remain},{festival_in_year_days}")
}

// fn is_leap_year(year: i32) -> bool {
//     year % 400 == 0 || (year % 100 != 0 && year % 4 == 0)
// }

fn get_days_mol7((year, month, day): (i32, i32, i32)) -> i32 {
    let y = year - 1;
    (y*365 %7 + y/4 - y/100 + y/400 + get_year_days(year, month, day) %7) %7
}

// 输入时间与上一年最后一天相差天数
fn get_year_days(year: i32, month: i32, day: i32) -> i32 {
    let month_day = [31, if is_leap_year!(year) {29} else {28}, 
                    31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days = day;
    for i in 0..(month-1) {
        days += month_day[i as usize];
    }
    days
}

fn get_festival_in_year_days(year: i32) -> i32 {
    let dates:Vec<(i32, i32)> = vec![
        (2, 5),
        (1, 24),
        (2, 12),
        (2, 1),
        (1, 22),
        (2, 9),
        (1, 29),
        (2, 18),
        (2, 7),
        (1, 26),
        (2, 14),
        (2, 3),
        (1, 23),
        (2, 10),
        (1, 31),
        (2, 19),
        (2, 8),
        (1, 28),
        (2, 16),
        (2, 5),
        (1, 25),
        (2, 12),
        (2, 1),
        (1, 22),
        (2, 10),
        (1, 29),
        (2, 17),
        (2, 6),
        (1, 26),
        (2, 13),
    ];
    if year < 2000 || year > 2029 {
        return 30;
    }
    get_year_days(year, dates[(year-2000)as usize].0, dates[(year-2000)as usize].1)
}
