use crate::{
    accessors::temporal::{day_to_second_duration::DayToSecondDuration, time::Time},
    adaptors::temporal_adaptors::{
        day_to_second_duration_adaptor::DayToSecondDurationAdaptor, time_adaptor::TimeAdaptor,
    },
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for zoned time values.
pub struct ZonedTime {
    time: Time,
    zone: DayToSecondDuration,
}

impl ZonedTime {
    /// Creates a new ZonedTime accessor.
    pub fn new(time: Box<dyn TimeAdaptor>, zone: Box<dyn DayToSecondDurationAdaptor>) -> Self {
        Self {
            time: Time::new(time),
            zone: DayToSecondDuration::new(zone),
        }
    }

    /// Returns the zoned-time's time component.
    pub fn time(&self) -> &Time {
        &self.time
    }

    /// Returns the zoned-time's zone component.
    pub fn zone(&self) -> &DayToSecondDuration {
        &self.zone
    }
}

impl SetEqualTo for ZonedTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this ZonedTime equal to another ZonedTime.
        todo!("Implement set_equal_to for ZonedTime");
    }
}

impl Accessor for ZonedTime {}
