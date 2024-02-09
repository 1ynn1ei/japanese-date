use chrono::prelude::*;
mod date;

fn main() {
    let local = Local::now();
    let era = date::retrieve_era(local);
    let weekday = date::retrieve_weekday(local);
    let month_day = date::retrieve_year_month(local);
    let time = date::retrieve_hour_minute(local);
    let output = format!("{time} {era}{month_day} ({weekday})");

    println!("{}", output);
}
