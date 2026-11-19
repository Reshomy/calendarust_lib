use super::{Anniversary, Task};
use chrono::{Datelike, NaiveDate};

pub enum Item {
    Anniversary(Anniversary),
    Task(Task),
}

impl Item {
    pub fn occurs_on(&self, date: NaiveDate) -> bool {
        match self {
            Item::Anniversary(anniversary) => {
                anniversary.month() == date.month() && anniversary.day() == date.day()
            }

            Item::Task(task) => task.date() == date,
        }
    }
}
