pub struct Anniversary {
    name: String,
    month: u32,
    day: u32,
}

impl Anniversary {
    pub fn new(name: String, month: u32, day: u32) -> Result<Self, AnniversaryError> {
        if name.trim().is_empty() {
            return Err(AnniversaryError::EmptyName);
        }

        if month == 0 || month > 12 || day == 0 || day > 31 {
            return Err(AnniversaryError::InvalidDate);
        }

        if chrono::NaiveDate::from_ymd_opt(2000, month, day).is_none() {
            return Err(AnniversaryError::InvalidDate);
        }

        Ok(Self { name, month, day })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnniversaryError {
    EmptyName,
    InvalidDate,
}
