use crate::{
    accessors::temporal::time::TimeError, adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
};

const TICKS_PER_MILLISECOND: u32 = 10;
const TICKS_PER_SECOND: u32 = 1000 * TICKS_PER_MILLISECOND;
const TICKS_PER_MINUTE: u32 = 60 * TICKS_PER_SECOND;
const TICKS_PER_HOUR: u32 = 60 * TICKS_PER_MINUTE;

/// An adaptor for times with a 100 microseconds resolution.
pub trait TimeAdaptor100Microseconds: TimeAdaptor {
    /// Returns the time stored as ticks.
    fn ticks(&self) -> Result<u32, TimeError>;

    /// Sets the time stored as ticks.
    fn set_ticks(&mut self, _ticks: u32) -> Result<(), TimeError>;

    fn can_return_time_as_nanos(&self) -> bool {
        true
    }

    fn nanos(&self) -> Result<u64, TimeError> {
        let ticks = self.ticks()?;
        Ok((ticks as u64) * 100_000)
    }

    /// Sets the time to the specified hour, minute, second, millisecond, and microsecond.
    fn do_set_time(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
    ) -> Result<(), TimeError> {
        let ticks = calulate_ticks(hour, minute, second, millisecond, microsecond);
        self.set_ticks(ticks)
    }

    /// Returns the current time as a tuple of (hour, minute, second, millisecond, microsecond, nanosecond).
    fn do_get_time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError> {
        let ticks = self.ticks()?;
        let hours = (ticks / TICKS_PER_HOUR) as u8;
        let minutes = ((ticks % TICKS_PER_HOUR) / TICKS_PER_MINUTE) as u8;
        let seconds = ((ticks % TICKS_PER_MINUTE) / TICKS_PER_SECOND) as u8;
        let milliseconds = (ticks % TICKS_PER_SECOND) / TICKS_PER_MILLISECOND;
        let microseconds = 100u32 * (ticks % TICKS_PER_MILLISECOND);
        Ok((
            hours,
            minutes,
            seconds,
            milliseconds as u16,
            microseconds as u16,
            0,
        ))
    }

    /// Returns the hour component of the time.
    fn do_get_hour(&self) -> Result<u8, TimeError> {
        let ticks = self.ticks()?;
        let hours = (ticks / TICKS_PER_HOUR) as u8;
        Ok(hours)
    }

    /// Returns the minute component of the time.
    fn do_get_minute(&self) -> Result<u8, TimeError> {
        let ticks = self.ticks()?;
        let minutes = ((ticks % TICKS_PER_HOUR) / TICKS_PER_MINUTE) as u8;
        Ok(minutes)
    }

    /// Returns the second component of the time.
    fn do_get_second(&self) -> Result<u8, TimeError> {
        let ticks = self.ticks()?;
        let seconds = ((ticks % TICKS_PER_MINUTE) / TICKS_PER_SECOND) as u8;
        Ok(seconds)
    }

    /// Returns the millisecond component of the time.
    fn do_get_millisecond(&self) -> Result<u16, TimeError> {
        let ticks = self.ticks()?;
        let milliseconds = (ticks % TICKS_PER_SECOND) / TICKS_PER_MILLISECOND;
        Ok(milliseconds as u16)
    }

    /// Returns the microsecond component of the time.
    fn do_get_microsecond(&self) -> Result<u16, TimeError> {
        let ticks = self.ticks()?;
        let microseconds = 100u32 * (ticks % TICKS_PER_MILLISECOND);
        Ok(microseconds as u16)
    }
}

/// Calculates the number of ticks for the specified time components.
fn calulate_ticks(hour: u8, minute: u8, second: u8, millisecond: u16, microsecond: u16) -> u32 {
    let ticks = if microsecond > 0 {
        let u_sec = (microsecond as f32 / 1000.0) * TICKS_PER_MILLISECOND as f32;
        u_sec as u32
    } else {
        0
    };
    (hour as u32) * TICKS_PER_HOUR
        + (minute as u32) * TICKS_PER_MINUTE
        + (second as u32) * TICKS_PER_SECOND
        + (millisecond as u32) * TICKS_PER_MILLISECOND
        + ticks
}
