use crate::accessors::temporal::date::{Date, DateError};

/// Sets the date from a string in an ISO 8601 format (YYYY-MM-DD), or (YYYYMMDD).
pub fn set_date_from_string(date: &mut Date, value: &str) -> Result<(), DateError> {
    let parts: Vec<&str> = value.split('-').collect();
    if parts.len() == 3 {
        let year_result: Result<u32, _> = parts[0].parse();
        if year_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Year '{}' is not a valid year number in date string '{}'",
                parts[0], value
            )));
        }
        let month_result: Result<u8, _> = parts[1].parse();
        if month_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Month '{}' is not a valid month number in date string '{}'",
                parts[1], value
            )));
        }
        let day_result: Result<u8, _> = parts[2].parse();
        if day_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Day '{}' is not a valid day number in date string '{}'",
                parts[2], value
            )));
        }
        date.set_date(
            year_result.unwrap() as u32,
            month_result.unwrap() as u32,
            day_result.unwrap() as u32,
        )
    } else if parts.len() == 1 && value.len() == 8 {
        let year_result: Result<u32, _> = value[0..4].parse();
        if year_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Year '{}' is not a valid year number in date string '{}'",
                &value[0..4],
                value
            )));
        }
        let month_result: Result<u8, _> = value[4..6].parse();
        if month_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Month '{}' is not a valid month number in date string '{}'",
                &value[4..6],
                value
            )));
        }
        let day_result: Result<u8, _> = value[6..8].parse();
        if day_result.is_err() {
            return Err(DateError::InvalidFormat(format!(
                "Day '{}' is not a valid day number in date string '{}'",
                &value[6..8],
                value
            )));
        }
        date.set_date(
            year_result.unwrap() as u32,
            month_result.unwrap() as u32,
            day_result.unwrap() as u32,
        )
    } else {
        Err(DateError::InvalidFormat(format!(
            "Date string '{}' is not in a valid ISO 8601 format  (YYYY-MM-DD), or (YYYYMMDD)",
            value
        )))
    }
}
