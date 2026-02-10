use crate::{
    accessors::temporal::time::TimeError, adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
};

const NANOS_PER_MICROSECOND: u64 = 1_000;
const NANOS_PER_MILLISECOND: u64 = 1_000 * NANOS_PER_MICROSECOND;
const NANOS_PER_SECOND: u64 = 1_000 * NANOS_PER_MILLISECOND;
const NANOS_PER_MINUTE: u64 = 60 * NANOS_PER_SECOND;
const NANOS_PER_HOUR: u64 = 60 * NANOS_PER_MINUTE;

/// An adaptor for times with nanosecond resolution.
pub trait TimeAdaptorNanosecond: TimeAdaptor {
    /// Sets the total nanoseconds representing the time.
    fn set_nanoseconds(&mut self, nanoseconds: u64) -> Result<(), TimeError>;

    /// Return the total nanoseconds representing the time.
    fn nanoseconds(&self) -> Result<u64, TimeError>;

    /// Sets the time to the specified hour, minute, second, millisecond, microsecond, and nanosecond.
    fn do_set_time(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
    ) -> Result<(), TimeError> {
        let total_nanoseconds =
            calculate_nanoseconds(hour, minute, second, millisecond, microsecond, nanosecond);
        self.set_nanoseconds(total_nanoseconds)
    }

    /// Returns the current time as a tuple of (hour, minute, second, millisecond, microsecond, nanosecond).
    fn do_get_time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let hour = (total_nanoseconds / NANOS_PER_HOUR) as u8;
        let minute = ((total_nanoseconds % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        let second = ((total_nanoseconds % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        let millisecond = ((total_nanoseconds % NANOS_PER_SECOND) / NANOS_PER_MILLISECOND) as u16;
        let microsecond =
            ((total_nanoseconds % NANOS_PER_MILLISECOND) / NANOS_PER_MICROSECOND) as u16;
        let nanosecond = (total_nanoseconds % NANOS_PER_MICROSECOND) as u16;
        Ok((hour, minute, second, millisecond, microsecond, nanosecond))
    }
    /// Returns the hour component of the time.
    fn do_get_hour(&self) -> Result<u8, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let hour = (total_nanoseconds / NANOS_PER_HOUR) as u8;
        Ok(hour)
    }

    /// Returns the minute component of the time.
    fn do_get_minute(&self) -> Result<u8, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let minute = ((total_nanoseconds % NANOS_PER_HOUR) / NANOS_PER_MINUTE) as u8;
        Ok(minute)
    }

    /// Returns the second component of the time.
    fn do_get_second(&self) -> Result<u8, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let second = ((total_nanoseconds % NANOS_PER_MINUTE) / NANOS_PER_SECOND) as u8;
        Ok(second)
    }

    /// Returns the millisecond component of the time.
    fn do_get_millisecond(&self) -> Result<u16, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let millisecond = (total_nanoseconds % NANOS_PER_SECOND) / NANOS_PER_MILLISECOND;
        Ok(millisecond as u16)
    }

    /// Returns the microsecond component of the time.
    fn do_get_microsecond(&self) -> Result<u16, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let microsecond = (total_nanoseconds % NANOS_PER_MILLISECOND) / NANOS_PER_MICROSECOND;
        Ok(microsecond as u16)
    }

    /// Returns the nanosecond component of the time.
    fn do_get_nanosecond(&self) -> Result<u16, TimeError> {
        let total_nanoseconds = self.nanoseconds()?;
        let nanosecond = total_nanoseconds % NANOS_PER_MICROSECOND;
        Ok(nanosecond as u16)
    }
}

/// Calculates the total nanoseconds from the given time components.
fn calculate_nanoseconds(
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
    microsecond: u16,
    nanosecond: u16,
) -> u64 {
    (hour as u64) * NANOS_PER_HOUR
        + (minute as u64) * NANOS_PER_MINUTE
        + (second as u64) * NANOS_PER_SECOND
        + (millisecond as u64) * NANOS_PER_MILLISECOND
        + (microsecond as u64) * NANOS_PER_MICROSECOND
        + (nanosecond as u64)
}
