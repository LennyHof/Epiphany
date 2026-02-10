use std::rc::Rc;

use crate::{
    accessors::temporal::day_to_second_duration::DayToSecondDurationError, adaptor::Adaptor,
    primitive_specs::duration_spec::DurationSpec,
};

/// An adaptor for day-second durations.
pub trait DayToSecondDurationAdaptor: Adaptor {
    /// Returns the duration's specification.
    fn spec(&self) -> &Rc<DurationSpec>;

    /// Sets the duration in days, hours, minutes, seconds, milliseconds, microseconds, and nanoseconds.
    fn set_duration(
        &mut self,
        days: i32,
        hours: i32,
        minutes: i32,
        seconds: i32,
        milliseconds: i32,
        microseconds: i32,
        nanoseconds: i32,
    ) -> Result<(), DayToSecondDurationError>;

    /// Returns the duration as a tuple of (days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds).
    fn duration(&self) -> Result<(i32, i32, i32, i32, i32, i32, i32), DayToSecondDurationError>;

    /// Returns the number of days in the duration.
    fn days(&self) -> Result<i32, DayToSecondDurationError> {
        let (days, _, _, _, _, _, _) = self.duration()?;
        Ok(days)
    }

    /// Returns the number of hours in the duration.
    fn hours(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, hours, _, _, _, _, _) = self.duration()?;
        Ok(hours)
    }

    /// Returns the number of minutes in the duration.
    fn minutes(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, _, minutes, _, _, _, _) = self.duration()?;
        Ok(minutes)
    }

    /// Returns the number of seconds in the duration.
    fn seconds(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, _, _, seconds, _, _, _) = self.duration()?;
        Ok(seconds)
    }

    /// Returns the number of milliseconds in the duration.
    fn milliseconds(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, _, _, _, milliseconds, _, _) = self.duration()?;
        Ok(milliseconds)
    }

    /// Returns the number of microseconds in the duration.
    fn microseconds(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, _, _, _, _, microseconds, _) = self.duration()?;
        Ok(microseconds)
    }

    /// Returns the number of nanoseconds in the duration.
    fn nanoseconds(&self) -> Result<i32, DayToSecondDurationError> {
        let (_, _, _, _, _, _, nanoseconds) = self.duration()?;
        Ok(nanoseconds)
    }
}
