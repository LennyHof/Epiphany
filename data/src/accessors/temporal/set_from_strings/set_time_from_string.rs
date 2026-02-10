use crate::accessors::temporal::time::{Time, TimeError};

pub fn set_time_from_string(time: &mut Time, time_str: &str) -> Result<(), TimeError> {
    let mut hour: u8 = 0;
    let mut minute: u8 = 0;
    let mut second: u8 = 0;
    let mut millisecond: u16 = 0;
    let mut microsecond: u16 = 0;
    let mut nanosecond: u16 = 0;

    let trimmed_time_str = time_str.trim_start_matches('T');
    let main_fraction_parts: Vec<&str> = trimmed_time_str.split(|c| c == '.' || c == ',').collect();
    let main = main_fraction_parts[0];
    let semicolon_parts: Vec<&str> = main.split(|c| c == ':').collect();

    if semicolon_parts.len() == 3 {
        let (h, m, s) = main_parts_from_collon_separated_string(time_str, semicolon_parts)?;
        hour = h;
        minute = m;
        second = s;
    } else if semicolon_parts.len() == 1 && main.len() == 6 {
        let (h, m, s) = main_parts_from_non_colon_separated_string(time_str, trimmed_time_str)?;
        hour = h;
        minute = m;
        second = s;
    } else {
        return Err(TimeError::InvalidFormat(time_str.to_string()));
    }

    if main_fraction_parts.len() == 2 {
        let fraction_str = main_fraction_parts[1];
        let (ms, us, ns) = parse_fraction(time_str, fraction_str)?;
        millisecond = ms;
        microsecond = us;
        nanosecond = ns;
    }

    return time.set_time(hour, minute, second, millisecond, microsecond, nanosecond);
}

/// Sets the time from a colon-separated string (e.g., "12:34:56.789").
fn main_parts_from_collon_separated_string(
    time_str: &str,
    parts: Vec<&str>,
) -> Result<(u8, u8, u8), TimeError> {
    let hour_result = parts[0].parse::<u8>();
    if hour_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Hour '{}' is not a valid hour number in time string '{}'",
            parts[0], time_str
        )));
    }
    let hour = hour_result.unwrap();

    let minute_result = parts[1].parse::<u8>();
    if minute_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Minute '{}' is not a valid minute number in time string '{}'",
            parts[1], time_str
        )));
    }
    let minute = minute_result.unwrap();

    let second_result = parts[2].parse::<u8>();
    if second_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Second '{}' is not a valid second number in time string '{}'",
            parts[2], time_str
        )));
    }
    let second = second_result.unwrap();

    return Ok((hour, minute, second));
}

/// Sets the time from a non-colon-separated string (e.g., "123456.789").
fn main_parts_from_non_colon_separated_string(
    time_str: &str,
    trimmed_time_str: &str,
) -> Result<(u8, u8, u8), TimeError> {
    let hour_result = trimmed_time_str[0..2].parse::<u8>();
    if hour_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Hour '{}' is not a valid hour number in time string '{}'",
            &trimmed_time_str[0..2],
            time_str
        )));
    }
    let hour = hour_result.unwrap();

    let minute_result = trimmed_time_str[2..4].parse::<u8>();
    if minute_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Minute '{}' is not a valid minute number in time string '{}'",
            &trimmed_time_str[2..4],
            time_str
        )));
    }
    let minute = minute_result.unwrap();

    let second_result = trimmed_time_str[4..6].parse::<u8>();
    if second_result.is_err() {
        return Err(TimeError::InvalidFormat(format!(
            "Second '{}' is not a valid second number in time string '{}'",
            &trimmed_time_str[4..6],
            time_str
        )));
    }
    let second = second_result.unwrap();

    return Ok((hour, minute, second));
}

/// Parses a fractional part of a second into milliseconds, microseconds, and nanoseconds.
fn parse_fraction(time_str: &str, fraction_str: &str) -> Result<(u16, u16, u16), TimeError> {
    let mut millisecond: u16 = 0;
    let mut microsecond: u16 = 0;
    let mut nanosecond: u16 = 0;
    let mut multipler = 100;
    for (index, c) in fraction_str.chars().enumerate() {
        let second_fraction: u16 = c.to_digit(10).ok_or_else(|| {
            TimeError::InvalidFormat(format!(
                "Second fraction '{}' is not a valid second fraction in time string '{}'",
                fraction_str, time_str
            ))
        })? as u16;
        match index {
            0..=2 => millisecond += second_fraction * multipler,
            3..=5 => microsecond += second_fraction * multipler,
            6..=8 => nanosecond += second_fraction * multipler,
            _ => {
                return Err(TimeError::InvalidFormat(format!(
                    "Second fraction '{}' is has too many digits in time string '{}'",
                    fraction_str, time_str
                )));
            }
        }
        multipler = if multipler == 1 { 100 } else { multipler / 10 };
    }
    Ok((millisecond, microsecond, nanosecond))
}
