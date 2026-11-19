use chrono::{NaiveDate, NaiveTime};

#[derive(Debug, PartialEq, Eq)]
pub enum TaskError {
    InvalidTimeRange,
}

pub struct Task {
    title: String,
    description: String,
    date: NaiveDate,
    start_time: NaiveTime,
    end_time: NaiveTime,
}

impl Task {
    pub fn new(
        title: String,
        description: String,
        date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Result<Self, TaskError> {
        if end_time <= start_time {
            return Err(TaskError::InvalidTimeRange);
        }

        Ok(Self {
            title,
            description,
            date,
            start_time,
            end_time,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn start_time(&self) -> NaiveTime {
        self.start_time
    }

    pub fn end_time(&self) -> NaiveTime {
        self.end_time
    }
}
