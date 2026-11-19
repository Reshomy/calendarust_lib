use chrono::{NaiveDate, NaiveTime};

use calendarust_lib::items::{Anniversary, Item, Task};
use calendarust_lib::planning::Planning;

#[test]
fn task_only_appears_on_its_scheduled_date() {
    let mut planning = Planning::new();

    let task_date = NaiveDate::from_ymd_opt(2026, 11, 19).unwrap();

    let task = Task::new(
        String::from("Team meeting"),
        String::from("Discuss the next release"),
        task_date,
        NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
    )
    .unwrap();

    planning.add_item(Item::Task(task));

    assert_eq!(planning.items_on(task_date).len(), 1);

    let next_day = NaiveDate::from_ymd_opt(2026, 11, 20).unwrap();

    assert_eq!(planning.items_on(next_day).len(), 0);
}

#[test]
fn anniversary_occurs_every_year() {
    let mut planning = Planning::new();

    let birthday = Anniversary::new(String::from("Alice's birthday"), 11, 19).unwrap();

    planning.add_item(Item::Anniversary(birthday));

    let date_2026 = NaiveDate::from_ymd_opt(2026, 11, 19).unwrap();
    let date_2027 = NaiveDate::from_ymd_opt(2027, 11, 19).unwrap();

    assert_eq!(planning.items_on(date_2026).len(), 1);
    assert_eq!(planning.items_on(date_2027).len(), 1);
}

#[test]
fn february_29_anniversary_only_occurs_in_leap_years() {
    let mut planning = Planning::new();

    let anniversary = Anniversary::new(String::from("Leap day anniversary"), 2, 29).unwrap();

    planning.add_item(Item::Anniversary(anniversary));

    let leap_year = NaiveDate::from_ymd_opt(2028, 2, 29).unwrap();
    let non_leap_year = NaiveDate::from_ymd_opt(2027, 2, 28).unwrap();

    assert_eq!(planning.items_on(leap_year).len(), 1);
    assert_eq!(planning.items_on(non_leap_year).len(), 0);
}
