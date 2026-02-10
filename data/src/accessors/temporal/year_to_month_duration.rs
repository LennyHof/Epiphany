use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    adaptors::temporal_adaptors::year_to_month_duration_adaptor::YearToMonthDurationAdaptor,
    primitive_def::Accessor,
    primitive_specs::duration_spec::{DurationResolution, DurationSpec},
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Accessor for year-month duration values.
pub struct YearToMonthDuration {
    adaptor: Box<dyn YearToMonthDurationAdaptor>,
}

impl YearToMonthDuration {
    /// Creates a new YearToMonthDuration accessor.
    pub fn new(adaptor: Box<dyn YearToMonthDurationAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the year-month duration's specification.
    pub fn spec(&self) -> &std::rc::Rc<DurationSpec> {
        self.adaptor.spec()
    }

    /// Sets the duration in years and months.
    pub fn set_duration(
        &mut self,
        years: i32,
        months: i32,
    ) -> Result<(), YearToMonthDurationError> {
        if let Some(resolution) = self.spec().resolution() {
            if months != 0 && *resolution == DurationResolution::Year {
                return Err(YearToMonthDurationError::ResolutionOutOfBounds(format!(
                    "Cannot set months with value {} on a year-to-month-duration with a Year resolution.",
                    months
                )));
            }
        }
        self.adaptor
            .set_duration(years, months)
            .map_err(YearToMonthDurationError::from)
    }

    /// Returns the duration as a tuple of (years, months).
    pub fn duration(&self) -> Result<(i32, i32), YearToMonthDurationError> {
        self.adaptor.duration()
    }

    /// Returns the number of years in the duration.
    pub fn years(&self) -> Result<i32, YearToMonthDurationError> {
        self.adaptor.years()
    }

    /// Returns the number of months in the duration.
    pub fn months(&self) -> Result<i32, YearToMonthDurationError> {
        self.adaptor.months()
    }
}

impl Accessor for YearToMonthDuration {}

impl Display for YearToMonthDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (years, months) = self.duration().unwrap();
        write!(f, "P{}Y{}M", years, months)
    }
}

impl Debug for YearToMonthDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (years, months) = self.duration().unwrap();
        write!(
            f,
            "YearToMonthDuration {{ years: {}, months: {} }}",
            years, months
        )
    }
}

impl SetEqualTo for YearToMonthDuration {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        let other_duration = other.duration()?;
        self.adaptor
            .set_duration(other_duration.0, other_duration.1)?;
        Ok(())
    }
}

impl PartialEq for YearToMonthDuration {
    fn eq(&self, other: &Self) -> bool {
        if self.adaptor.stores_duration_as_months() && other.adaptor.stores_duration_as_months() {
            self.adaptor.total_months().unwrap() == other.adaptor.total_months().unwrap()
        } else {
            self.duration().unwrap() == other.duration().unwrap()
        }
    }
}

impl Eq for YearToMonthDuration {}

impl PartialOrd for YearToMonthDuration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.adaptor.stores_duration_as_months() && other.adaptor.stores_duration_as_months() {
            self.adaptor
                .total_months()
                .unwrap()
                .partial_cmp(&other.adaptor.total_months().unwrap())
        } else {
            self.duration()
                .unwrap()
                .partial_cmp(&other.duration().unwrap())
        }
    }
}

impl Ord for YearToMonthDuration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.adaptor.stores_duration_as_months() && other.adaptor.stores_duration_as_months() {
            self.adaptor
                .total_months()
                .unwrap()
                .cmp(&other.adaptor.total_months().unwrap())
        } else {
            self.duration().unwrap().cmp(&other.duration().unwrap())
        }
    }
}

impl Hash for YearToMonthDuration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.adaptor.stores_duration_as_months() {
            self.adaptor.total_months().unwrap().hash(state);
        } else {
            self.duration().unwrap().hash(state);
        }
    }
}

/// Errors that can occur when working with year-month durations.
#[derive(Debug, PartialEq)]
pub enum YearToMonthDurationError {
    /// A provider error.
    ProviderError(ProviderError),
    /// Indicates that the resolution is out of bounds.
    ResolutionOutOfBounds(String),
    /// Indicates an invalid format error.
    InvalidFormat(String),
}

impl Display for YearToMonthDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            YearToMonthDurationError::ProviderError(err) => write!(f, "Provider error: {}", err),
            YearToMonthDurationError::ResolutionOutOfBounds(msg) => {
                write!(f, "Resolution out of bounds: {}", msg)
            }
            YearToMonthDurationError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

impl std::error::Error for YearToMonthDurationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            YearToMonthDurationError::ProviderError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ProviderError> for YearToMonthDurationError {
    fn from(error: ProviderError) -> Self {
        YearToMonthDurationError::ProviderError(error)
    }
}

impl From<YearToMonthDurationError> for SetEqualToError {
    fn from(error: YearToMonthDurationError) -> Self {
        SetEqualToError::YearToMonthDurationError(error)
    }
}
