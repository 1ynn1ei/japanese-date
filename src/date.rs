use chrono::{DateTime, Datelike, Local, Timelike, Weekday};

pub fn retrieve_era(date: DateTime<Local>) -> String {
    let year = date.year();
    let mut era = String::new();
    let mut era_year = 0;
    // TODO: obviously era's dont start at the start of a year, so we need
    // more control here with the matching (probably just match utc timestamp)
    match year {
        2019..=i32::MAX => {
            era_year = year - 2019;
            era = String::from("令和");
        },
        1989..=2018 => {
            era_year = year - 1989;
            era = String::from("平成");
        },
        _ => {} 
    }
    if era_year == 1 {
        format!("{era}元年")
    } else {
        format!("{era}{era_year}年")
    }
}

pub fn retrieve_weekday(date: DateTime<Local>) -> String {
    let weekday = date.weekday();
    match weekday {
        Weekday::Sun => String::from("日"),
        Weekday::Mon => String::from("月"),
        Weekday::Tue => String::from("火"),
        Weekday::Wed => String::from("水"),
        Weekday::Thu => String::from("木"),
        Weekday::Fri => String::from("金"),
        Weekday::Sat => String::from("土"),
    }
}

pub fn retrieve_year_month(date: DateTime<Local>) -> String {
    let month = date.month();
    let day = date.day();
    format!("{month}月{day}日")
}

pub fn retrieve_hour_minute(date: DateTime<Local>) -> String {
    let (gogo, mut hour) = date.hour12();
    hour %= 12;
    let minute = date.minute();
    let second = date.second();
    let prefix = if !gogo { "午前" } else { "午後" };
    format!("{prefix}{hour}時{minute}分{second}秒")
}
