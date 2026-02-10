use std::{
    fmt::{Debug, Display},
    hash::Hash,
    rc::Rc,
};

use crate::{
    adaptors::temporal_adaptors::date_adaptor::DateAdaptor,
    primitive_def::Accessor,
    primitive_specs::date_spec::DateSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Enumeration of the days of the week.
#[derive(Debug, PartialEq)]
pub enum DayOfWeek {
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let day_str = match self {
            DayOfWeek::Sunday => "Sunday",
            DayOfWeek::Monday => "Monday",
            DayOfWeek::Tuesday => "Tuesday",
            DayOfWeek::Wednesday => "Wednesday",
            DayOfWeek::Thursday => "Thursday",
            DayOfWeek::Friday => "Friday",
            DayOfWeek::Saturday => "Saturday",
        };
        write!(f, "{}", day_str)
    }
}

/// Accessor for date values.
/// <p>
/// Date values represent a date on the Gregorian calendar, with day precision.
/// Date values range from January 1, 0001 Anno Domini (Common Era) to December 31, 9999 Anno
/// Domini (Common Era).
/// </p>
pub struct Date {
    adaptor: Box<dyn DateAdaptor>,
}

impl Date {
    /// Creates a new Date accessor.
    pub fn new(adaptor: Box<dyn DateAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the date's specification.
    pub fn spec(&self) -> &Rc<DateSpec> {
        self.adaptor.spec()
    }

    /// Sets the date's value to the specified year, month, and day.
    ///
    /// ```rust
    /// use data::data_spec_builders::date_spec_builder::DateSpecBuilder;
    /// use data::variable::Variable;
    /// use data::accessors::temporal::date::Date;
    ///
    /// let mut var = Variable::new(&DateSpecBuilder::new().build());
    ///     let date_mut = var.date_mut();
    ///
    ///     let result = date_mut.set_date(2024, 6, 15);
    ///     assert!(result.is_ok());
    ///
    ///     let (year, month, day) = date_mut.date().unwrap();
    ///     assert_eq!(year, 2024);
    ///     assert_eq!(month, 6);
    ///     assert_eq!(day, 15);
    ///
    /// ```
    pub fn set_date(&mut self, year: u32, month: u32, day: u32) -> Result<(), DateError> {
        if (year < 1) || (year > 9999) {
            return Err(DateError::YearOutOfBounds(year));
        }
        if (month < 1) || (month > 12) {
            return Err(DateError::MonthOutOfBounds(month));
        }
        let is_leap = is_leap_year(year);
        let days_in_month = days_in_month(is_leap, month);
        if day < 1 || day > days_in_month {
            return Err(DateError::DayOutOfBounds(
                day,
                month_name(month).to_string(),
                days_in_month,
            ));
        }
        self.adaptor.set_date(year, month, day)?;
        Ok(())
    }

    /// Sets the date's value from a string in an ISO 8601 format: (YYYY-MM-DD), or (YYYYMMDD).
    /// Note that a truncated date such as "2024-06" is not supported and will result in an error.
    ///
    /// # Examples
    ///
    /// Set the date to Jun 15, 2024 using dashes:
    /// ```rust
    /// use data::data_spec_builders::date_spec_builder::DateSpecBuilder;
    /// use data::variable::Variable;
    ///
    /// let mut var = Variable::new(&DateSpecBuilder::new().build());
    ///     let date_mut = var.date_mut();
    ///
    ///     let result = date_mut.set_from_string("2024-06-15");
    ///     assert!(result.is_ok());
    ///
    ///     let (year, month, day) = date_mut.date().unwrap();
    ///     assert_eq!(year, 2024);
    ///     assert_eq!(month, 6);
    ///     assert_eq!(day, 15);
    ///
    /// ```
    /// Set the date to Jun 15, 2024 without using dashes:
    /// ```rust
    /// use data::data_spec_builders::date_spec_builder::DateSpecBuilder;
    /// use data::variable::Variable;
    ///
    /// let mut var = Variable::new(&DateSpecBuilder::new().build());
    ///     let date_mut = var.date_mut();
    ///
    ///     let result = date_mut.set_from_string("20240615");
    ///     assert!(result.is_ok());
    ///
    ///     let (year, month, day) = date_mut.date().unwrap();
    ///     assert_eq!(year, 2024);
    ///     assert_eq!(month, 6);
    ///     assert_eq!(day, 15);
    ///
    /// ```
    pub fn set_from_string(&mut self, date_str: &str) -> Result<(), DateError> {
        crate::accessors::temporal::set_from_strings::set_date_from_string::set_date_from_string(
            self, date_str,
        )
    }

    /// Returns the date as a tuple of (year, month, day).
    pub fn date(&self) -> Result<(u32, u32, u32), DateError> {
        self.adaptor.date()
    }

    /// Returns the year component of the date.
    pub fn year(&self) -> Result<u32, DateError> {
        self.adaptor.year()
    }

    /// Returns the month component of the date.
    pub fn month(&self) -> Result<u32, DateError> {
        self.adaptor.month()
    }

    /// Returns the day component of the date.
    pub fn day(&self) -> Result<u32, DateError> {
        self.adaptor.day()
    }

    /// Returns the day of the week on which this date falls.
    pub fn day_of_week(&self) -> Result<DayOfWeek, DateError> {
        let (year, month, day) = self.date().unwrap();
        let q = day as i32;
        let m = if month < 3 { month + 12 } else { month } as i32;
        let k = if month < 3 { year - 1 } else { year } as i32 % 100;
        let j = if month < 3 { year - 1 } else { year } as i32 / 100;

        let h = (q + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;

        match h {
            0 => Ok(DayOfWeek::Saturday),
            1 => Ok(DayOfWeek::Sunday),
            2 => Ok(DayOfWeek::Monday),
            3 => Ok(DayOfWeek::Tuesday),
            4 => Ok(DayOfWeek::Wednesday),
            5 => Ok(DayOfWeek::Thursday),
            6 => Ok(DayOfWeek::Friday),
            _ => unreachable!(),
        }
    }

    /// Returns the number of days from January 1 to the date represented in this date.
    pub fn day_of_year(&self) -> Result<u32, DateError> {
        let (year, month, day) = self.date()?;
        let mut day_of_year = day;
        let is_leap = is_leap_year(year);
        for m in 1..month {
            day_of_year += days_in_month(is_leap, m);
        }
        Ok(day_of_year)
    }
}

impl Accessor for Date {}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day) = self.date().unwrap();
        write!(f, "{:04}-{:02}-{:02}", year, month, day)
    }
}

impl Debug for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (year, month, day) = self.date().unwrap();
        write!(
            f,
            "Date {{ year: {}, month: {}, day: {} }}",
            year, month, day
        )
    }
}

impl SetEqualTo for Date {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        if self.adaptor.stores_date_as_days() && other.adaptor.stores_date_as_days() {
            self.adaptor.set_days(other.adaptor.days()?)?;
            return Ok(());
        }
        let other_date = other.date()?;
        self.set_date(other_date.0, other_date.1, other_date.2)?;
        Ok(())
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        if self.adaptor.stores_date_as_days() && other.adaptor.stores_date_as_days() {
            return self.adaptor.days().unwrap() == other.adaptor.days().unwrap();
        }
        self.date() == other.date()
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.adaptor.stores_date_as_days() && other.adaptor.stores_date_as_days() {
            return self
                .adaptor
                .days()
                .unwrap()
                .partial_cmp(&other.adaptor.days().unwrap());
        }
        self.date().unwrap().partial_cmp(&other.date().unwrap())
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hash for Date {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.adaptor.stores_date_as_days() {
            self.adaptor.days().unwrap().hash(state);
        } else {
            let (year, month, day) = self.date().unwrap();
            year.hash(state);
            month.hash(state);
            day.hash(state);
        }
    }
}

/// Determines if a given year is a leap year.
pub(crate) fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

/// Returns the number of days in a given month of a given year.
pub(crate) fn days_in_month(is_leap_year: bool, month: u32) -> u32 {
    const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    const DAYS_IN_MONTH_LEAP: [u32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if is_leap_year {
        return DAYS_IN_MONTH_LEAP[(month - 1) as usize];
    }
    DAYS_IN_MONTH[(month - 1) as usize]
}

/// Returns the name of the month for the given month number.
pub fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Invalid Month",
    }
}

/// Errors that can occur when working with dates.
#[derive(Debug, PartialEq)]
pub enum DateError {
    /// A provider error.
    ProviderError(ProviderError),
    /// Year out of bounds.
    YearOutOfBounds(u32),
    /// Month out of bounds.
    MonthOutOfBounds(u32),
    /// Day out of bounds.
    DayOutOfBounds(u32, String, u32),
    /// Invalid date format.
    InvalidFormat(String),
}

impl Display for DateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateError::ProviderError(err) => write!(f, "Provider error: {}", err),
            DateError::YearOutOfBounds(year) => {
                write!(f, "Year value {} is out of bounds (1 to 9999).", year)
            }
            DateError::MonthOutOfBounds(month) => {
                write!(f, "Month value {} is out of bounds (1 to 12).", month)
            }
            DateError::DayOutOfBounds(day, month_name, max_day) => {
                write!(
                    f,
                    "Day value {} is out of bounds for month {}. Valid range is 1 to {}.",
                    day, month_name, max_day
                )
            }
            DateError::InvalidFormat(msg) => {
                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for DateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DateError::ProviderError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ProviderError> for DateError {
    fn from(error: ProviderError) -> Self {
        DateError::ProviderError(error)
    }
}

impl From<DateError> for SetEqualToError {
    fn from(error: DateError) -> Self {
        SetEqualToError::DateError(error)
    }
}
