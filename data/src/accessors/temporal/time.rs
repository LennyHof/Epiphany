use std::{
    fmt::{Debug, Display},
    hash::Hash,
};

use crate::{
    adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
    primitive_def::Accessor,
    primitive_specs::time_spec::{TimeResolution, TimeSpec},
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for time-of-day values.
/// <p>
/// time values represent a time of day without reference to a specific date or time zone.
/// The time is represented in the 24-hour clock format, ranging from 00:00:00.000000000 to
/// 23:59:59.999999999 with resolution determined by the underlying adaptor.
/// </p>
pub struct Time {
    adaptor: Box<dyn TimeAdaptor>,
}

impl Time {
    /// Creates a new Time accessor.
    pub fn new(adaptor: Box<dyn TimeAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the time's specification.
    pub fn spec(&self) -> &std::rc::Rc<TimeSpec> {
        self.adaptor.spec()
    }

    /// Sets the time to the specified hours, minutes, seconds, milliseconds, and microseconds.
    pub fn set_time(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
    ) -> Result<(), TimeError> {
        if hour >= 24 {
            return Err(TimeError::HourOutOfBounds(hour));
        }
        if minute >= 60 {
            return Err(TimeError::MinuteOutOfBounds(minute));
        }
        if second >= 60 {
            return Err(TimeError::SecondOutOfBounds(second));
        }
        if millisecond >= 1_000 {
            return Err(TimeError::MillisecondOutOfBounds(millisecond));
        }
        if microsecond >= 1_000 {
            return Err(TimeError::MicrosecondOutOfBounds(microsecond));
        }
        if nanosecond >= 1_000 {
            return Err(TimeError::NanosecondOutOfBounds(nanosecond));
        }
        if let Some(resolution) = self.spec().resolution() {
            if millisecond > 0 && *resolution < TimeResolution::Millisecond {
                return Err(TimeError::ResolutionOutOfBounds(format!(
                    "Cannot set milliseconds with value {} on time with {} resolution.",
                    millisecond, *resolution
                )));
            }
            if microsecond > 0 {
                if microsecond % 100 == 0 {
                    if *resolution < TimeResolution::Microsecond100 {
                        return Err(TimeError::ResolutionOutOfBounds(format!(
                            "Cannot set microseconds with value {} on time with {} resolution.",
                            microsecond, *resolution
                        )));
                    }
                } else if *resolution < TimeResolution::Microsecond {
                    return Err(TimeError::ResolutionOutOfBounds(format!(
                        "Cannot set microseconds with value {} on time with {} resolution.",
                        microsecond, *resolution
                    )));
                }
            }
            if nanosecond > 0 && *resolution < TimeResolution::Nanosecond {
                return Err(TimeError::ResolutionOutOfBounds(format!(
                    "Cannot set nanoseconds with value {} on time with {} resolution.",
                    nanosecond, *resolution
                )));
            }
        }

        self.adaptor
            .set_time(hour, minute, second, millisecond, microsecond, nanosecond)
    }

    /// Sets the time via a tuple of (hours, minutes, seconds, milliseconds, microseconds, nanoseconds).
    pub fn set_via_tuple(
        &mut self,
        time_components: (u8, u8, u8, u16, u16, u16),
    ) -> Result<(), TimeError> {
        let (hours, minutes, seconds, milliseconds, microseconds, nanoseconds) = time_components;
        self.set_time(
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        )
    }

    /// Sets the time from a string in the in an ISO 8601 format "HH:MM:SS.mmmuuuNNN".
    pub fn set_from_string(&mut self, time_str: &str) -> Result<(), TimeError> {
        crate::accessors::temporal::set_from_strings::set_time_from_string::set_time_from_string(
            self, time_str,
        )
    }

    /// Returns the current time as a tuple of (hours, minutes, seconds, milliseconds, microseconds, nanoseconds).
    pub fn time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError> {
        self.adaptor.time()
    }

    /// Returns the hour component of the time.
    pub fn hour(&self) -> Result<u8, TimeError> {
        self.adaptor.hour()
    }

    /// Returns the minute component of the time.
    pub fn minute(&self) -> Result<u8, TimeError> {
        self.adaptor.minute()
    }

    /// Returns the second component of the time.
    pub fn second(&self) -> Result<u8, TimeError> {
        self.adaptor.second()
    }

    /// Returns the millisecond component of the time.
    pub fn millisecond(&self) -> Result<u16, TimeError> {
        self.adaptor.millisecond()
    }

    /// Returns the microsecond component of the time.
    pub fn microsecond(&self) -> Result<u16, TimeError> {
        self.adaptor.microsecond()
    }

    /// Returns the nanosecond component of the time.   
    pub fn nanosecond(&self) -> Result<u16, TimeError> {
        self.adaptor.nanosecond()
    }
}

impl Accessor for Time {}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hours, minutes, seconds, milliseconds, microseconds, nanoseconds) =
            self.time().unwrap();
        if milliseconds == 0 && microseconds == 0 && nanoseconds == 0 {
            // no fractional part, so we don't display a fractional part
            write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            // we have a fractional part, so we display it
            let mut fractional =
                format!("{:03}{:03}{:03}", milliseconds, microseconds, nanoseconds);
            // trim trailing zeros for readability
            fractional = fractional.trim_end_matches('0').to_string();
            write!(
                f,
                "{:02}:{:02}:{:02}.{}",
                hours, minutes, seconds, fractional
            )
        }
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (hours, minutes, seconds, milliseconds, microseconds, nanoseconds) =
            self.time().unwrap();
        write!(
            f,
            "Time {{ hour: {:02}, minute: {:02}, second: {:02}, millisecond: {:03}, microsecond: {:03}, nanosecond: {:03} }}",
            hours, minutes, seconds, milliseconds, microseconds, nanoseconds
        )
    }
}

impl SetEqualTo for Time {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        let (hours, minutes, seconds, milliseconds, microseconds, nanoseconds) = other.time()?;
        self.set_time(
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
        )?;
        Ok(())
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        if self.adaptor.can_return_time_as_nanos() && other.adaptor.can_return_time_as_nanos() {
            match (self.adaptor.nanos(), other.adaptor.nanos()) {
                (Ok(self_nanos), Ok(other_nanos)) => return self_nanos == other_nanos,
                _ => {}
            }
        }
        self.time() == other.time()
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.adaptor.can_return_time_as_nanos() && other.adaptor.can_return_time_as_nanos() {
            match (self.adaptor.nanos(), other.adaptor.nanos()) {
                (Ok(self_nanos), Ok(other_nanos)) => return self_nanos.partial_cmp(&other_nanos),
                _ => {}
            }
        }
        Some(self.time().unwrap().cmp(&other.time().unwrap()))
    }
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Hash for Time {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.adaptor.can_return_time_as_nanos() {
            if let Ok(nanos) = self.adaptor.nanos() {
                nanos.hash(state);
                return;
            }
        }
        if let Ok((hours, minutes, seconds, milliseconds, microseconds, nanoseconds)) = self.time()
        {
            hours.hash(state);
            minutes.hash(state);
            seconds.hash(state);
            milliseconds.hash(state);
            microseconds.hash(state);
            nanoseconds.hash(state);
        }
    }
}

/// An error that can occur when working with time values.
#[derive(Debug, PartialEq)]
pub enum TimeError {
    /// A provider error.
    ProviderError(ProviderError),
    /// Indicates that the hour value is out of bounds.
    HourOutOfBounds(u8),
    /// Indicates that the minute value is out of bounds.
    MinuteOutOfBounds(u8),
    /// Indicates that the second value is out of bounds.
    SecondOutOfBounds(u8),
    /// Indicates that the millisecond value is out of bounds.
    MillisecondOutOfBounds(u16),
    /// Indicates that the microsecond value is out of bounds.
    MicrosecondOutOfBounds(u16),
    /// Indicates that the nanosecond value is out of bounds.
    NanosecondOutOfBounds(u16),
    /// Indicates that the resolution is out of bounds.
    ResolutionOutOfBounds(String),
    /// Indicates an invalid format error.
    InvalidFormat(String),
}

impl Display for TimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeError::ProviderError(err) => write!(f, "Provider error: {}", err),
            TimeError::HourOutOfBounds(hour) => {
                write!(f, "Hour value {} is out of bounds (0-23).", hour)
            }
            TimeError::MinuteOutOfBounds(minute) => {
                write!(f, "Minute value {} is out of bounds (0-59).", minute)
            }
            TimeError::SecondOutOfBounds(second) => {
                write!(f, "Second value {} is out of bounds (0-59).", second)
            }
            TimeError::MillisecondOutOfBounds(ms) => {
                write!(f, "Millisecond value {} is out of bounds (0-999).", ms)
            }
            TimeError::MicrosecondOutOfBounds(us) => {
                write!(f, "Microsecond value {} is out of bounds (0-999).", us)
            }
            TimeError::NanosecondOutOfBounds(ns) => {
                write!(f, "Nanosecond value {} is out of bounds (0-999).", ns)
            }
            TimeError::ResolutionOutOfBounds(msg) => write!(f, "Resolution out of bounds: {}", msg),
            TimeError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for TimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TimeError::ProviderError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<TimeError> for SetEqualToError {
    fn from(error: TimeError) -> Self {
        SetEqualToError::TimeError(error)
    }
}
