use chrono::{Datelike, NaiveDate};

use super::{Anniversary, Task};

pub enum ItemKind {
    Anniversary(Anniversary),
    Task(Task),
}

pub struct Item {
    id: u64,
    kind: ItemKind,
}

impl Item {
    pub fn new(id: u64, kind: ItemKind) -> Self {
        Self { id, kind }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn kind(&self) -> &ItemKind {
        &self.kind
    }

    pub fn occurs_on(&self, date: NaiveDate) -> bool {
        match &self.kind {
            ItemKind::Anniversary(anniversary) => {
                anniversary.month() == date.month() && anniversary.day() == date.day()
            }

            ItemKind::Task(task) => task.date() == date,
        }
    }
}
