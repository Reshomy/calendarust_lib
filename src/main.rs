use calendar_tasks::items::Anniversary;
use chrono::NaiveDate;

fn main() {
    let birthday = Anniversary::new(
        String::from("John's birthday"),
        NaiveDate::from_ymd_opt(2026, 11, 19).unwrap(),
    );

    println!("{}", birthday.name());
    println!("{}", birthday.date());
}
