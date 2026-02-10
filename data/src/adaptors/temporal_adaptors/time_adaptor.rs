use std::rc::Rc;

use crate::{
    accessors::temporal::time::TimeError, adaptor::Adaptor, primitive_specs::time_spec::TimeSpec,
};

/// An adaptor for times.
pub trait TimeAdaptor: Adaptor {
    /// Returns the time's specification.
    fn spec(&self) -> &Rc<TimeSpec>;

    /// Indicates whether the time can be returned as nanoseconds.
    fn can_return_time_as_nanos(&self) -> bool {
        false
    }

    /// Returns the time in nanoseconds if the time can be returned as nanoseconds.
    fn nanos(&self) -> Result<u64, TimeError> {
        unimplemented!()
    }

    /// Sets the time to the specified hour, minute, second, millisecond, microsecond, and nanosecond.
    fn set_time(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
    ) -> Result<(), TimeError>;

    /// Sets the time to the current system time.
    fn set_to_now(&mut self) -> Result<(), TimeError> {
        unimplemented!()
    }

    /// Returns the current time as a tuple of (hour, minute, second, millisecond, microsecond, nanosecond).
    fn time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError>;

    /// Returns the hour component of the time.
    fn hour(&self) -> Result<u8, TimeError> {
        let (hour, _, _, _, _, _) = self.time()?;
        Ok(hour)
    }

    /// Returns the minute component of the time.
    fn minute(&self) -> Result<u8, TimeError> {
        let (_, minute, _, _, _, _) = self.time()?;
        Ok(minute)
    }

    /// Returns the second component of the time.
    fn second(&self) -> Result<u8, TimeError> {
        let (_, _, second, _, _, _) = self.time()?;
        Ok(second)
    }

    /// Returns the millisecond component of the time.
    fn millisecond(&self) -> Result<u16, TimeError> {
        let (_, _, _, millisecond, _, _) = self.time()?;
        Ok(millisecond)
    }

    /// Returns the microsecond component of the time.
    fn microsecond(&self) -> Result<u16, TimeError> {
        let (_, _, _, _, microsecond, _) = self.time()?;
        Ok(microsecond)
    }

    /// Returns the nanosecond component of the time.   
    fn nanosecond(&self) -> Result<u16, TimeError> {
        let (_, _, _, _, _, nanosecond) = self.time()?;
        Ok(nanosecond)
    }
}
