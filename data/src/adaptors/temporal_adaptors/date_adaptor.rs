use crate::{
    accessors::temporal::date::{DateError, days_in_month, is_leap_year},
    adaptor::Adaptor,
    primitive_specs::date_spec::DateSpec,
};
use std::rc::Rc;

/// An adaptor for dates on the Gregorian calendar, with day precision.
/// <p>
/// The date is represented as the number of days from January 1, 0001 A.D. (C.E.) to year 9999.
/// </p>
///
/// It is expected that an implementation will override the `set_days` and `days` methods to provide
/// the actual storage and retrieval of the date value as a u32 without overriding `set_date` and `date`,
/// or override the `set_date` and `date` methods without overriding `set_days` and `days`.
///
pub trait DateAdaptor: Adaptor {
    /// Returns the date's specification.
    fn spec(&self) -> &Rc<DateSpec>;

    /// Returns true if the date is stored as total days by this adaptor; false otherwise.
    fn stores_date_as_days(&self) -> bool;

    /// Sets the date value to the specified days (Number of days in this date, an integer number
    /// of whole days from January 1,
    /// 0001 A.D. (C.E.) to the year 9999.).
    fn set_days(&mut self, _days: u32) -> Result<(), DateError> {
        unimplemented!()
    }

    /// Returns the date value as days (Number of days in this date, an integer number
    /// of whole days from January 1,
    /// 0001 A.D. (C.E.) to the year 9999.).
    fn days(&self) -> Result<u32, DateError> {
        unimplemented!()
    }

    /// Sets the date to the specified year, month, and day.
    fn set_date(&mut self, year: u32, month: u32, day: u32) -> Result<(), DateError> {
        if !self.stores_date_as_days() {
            unimplemented!()
        }
        let y = year - 1;
        let mut days = (365u32 * y) + (y / 4u32) - (y / 100u32) + (y / 400u32);
        days += days_before_month(is_leap_year(year), month) + day - 1;
        self.set_days(days)
    }

    /// Returns the current date as a tuple of (year, month, day).
    fn date(&self) -> Result<(u32, u32, u32), DateError> {
        if !self.stores_date_as_days() {
            unimplemented!()
        }
        let mut days = self.days()?;
        let year = year_from_days(&mut days);
        let month = month_from_days(is_leap_year(year), &mut days);
        Ok((
            year,
            month,
            days + 1, /* to got to one based from zero based */
        ))
    }

    /// Returns the year component of the current date.
    fn year(&self) -> Result<u32, DateError> {
        if self.stores_date_as_days() {
            let mut days = self.days()?;
            return Ok(year_from_days(&mut days));
        }
        let (year, _, _) = self.date()?;
        Ok(year)
    }
    /// Returns the month component of the current date.
    fn month(&self) -> Result<u32, DateError> {
        if self.stores_date_as_days() {
            let mut days = self.days()?;
            let year = year_from_days(&mut days);
            return Ok(month_from_days(is_leap_year(year), &mut days));
        }
        let (_, month, _) = self.date()?;
        Ok(month)
    }

    /// Returns the day component of the current date.
    fn day(&self) -> Result<u32, DateError> {
        if self.stores_date_as_days() {
            let mut days = self.days()?;
            let year = year_from_days(&mut days);
            let _ = month_from_days(is_leap_year(year), &mut days);
            return Ok(days + 1 /* to got to one based from zero based */);
        }
        let (_, _, day) = self.date()?;
        Ok(day)
    }
}

/// Returns the number of days before the given month in a year according to if it is a leap year.
fn days_before_month(is_leap_year: bool, month: u32) -> u32 {
    const PROCEDING_DAYS_FOR_MONTH: [u32; 12] =
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    const PROCEDING_DAYS_FOR_MONTH_LEAP: [u32; 12] =
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335];

    let days = if is_leap_year {
        PROCEDING_DAYS_FOR_MONTH_LEAP[(month - 1) as usize]
    } else {
        PROCEDING_DAYS_FOR_MONTH[(month - 1) as usize]
    };
    days
}
const DAYS_NON_LEAP_YEAR: u32 = 365;
const DAYS_PER_400_YEARS: u32 = (400 * DAYS_NON_LEAP_YEAR) + 97/* 97 leap years */;
const DAYS_PER_100_YEARS: u32 = (100 * DAYS_NON_LEAP_YEAR) + 24/* 24 leap years */;
const DAYS_PER_4_YEARS: u32 = (4 * DAYS_NON_LEAP_YEAR) + 1/* 1 leap year */;

/// Returns the year component from the given days.
fn year_from_days(days: &mut u32) -> u32 {
    // calculate number of 400 years, 100 years, 4 years, and remaining years
    // adjusting days for each calculation
    let number_of_400_years = *days / DAYS_PER_400_YEARS;
    *days -= number_of_400_years * DAYS_PER_400_YEARS;

    let number_of_100_years = *days / DAYS_PER_100_YEARS;
    *days -= number_of_100_years * DAYS_PER_100_YEARS;

    let number_of_4_years = *days / DAYS_PER_4_YEARS;
    *days -= number_of_4_years * DAYS_PER_4_YEARS;

    let mut years = *days / 365;
    if years == 4 {
        // This is the last day of a leap year
        years -= 1;
    }
    *days -= years * DAYS_NON_LEAP_YEAR;

    // now calculate the final year
    let year = (number_of_400_years * 400)
        + (number_of_100_years * 100)
        + (number_of_4_years * 4)
        + years
        + 1/* to got to one based from zero based */;

    year
}

/// Returns the month component from the given days.
fn month_from_days(is_leap_year: bool, days: &mut u32) -> u32 {
    let mut month = 1;
    while *days >= days_in_month(is_leap_year, month) {
        *days -= days_in_month(is_leap_year, month);
        month += 1/* to got to one based from zero based */;
    }
    month
}
