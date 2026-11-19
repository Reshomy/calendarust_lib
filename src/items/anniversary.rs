use chrono::NaiveDate;

pub struct Anniversary {
    name: String,
    date: NaiveDate,
}

impl Anniversary {
    pub fn new(name: String, date: NaiveDate) -> Self {
        Self { name, date }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }
}
