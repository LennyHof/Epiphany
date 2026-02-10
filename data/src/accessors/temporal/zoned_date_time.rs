use crate::{
    accessors::temporal::{date::Date, zoned_time::ZonedTime},
    adaptors::temporal_adaptors::{
        date_adaptor::DateAdaptor, day_to_second_duration_adaptor::DayToSecondDurationAdaptor,
        time_adaptor::TimeAdaptor,
    },
    primitive_def::Accessor,
    set_equal_to::SetEqualTo,
};

/// Accessor for date-time values.
pub struct ZonedDateTime {
    date: Date,
    zoned_time: ZonedTime,
}
impl ZonedDateTime {
    /// Creates a new ZonedDateTime accessor.
    pub fn new(
        date_adaptor: Box<dyn DateAdaptor>,
        time_adaptor: Box<dyn TimeAdaptor>,
        zone_adaptor: Box<dyn DayToSecondDurationAdaptor>,
    ) -> Self {
        Self {
            date: Date::new(date_adaptor),
            zoned_time: ZonedTime::new(time_adaptor, zone_adaptor),
        }
    }

    /// Returns the zoned-date-time's date component.
    pub fn date(&self) -> &Date {
        &self.date
    }

    /// Returns the zoned-date-time's zoned-time component.
    pub fn zoned_time(&self) -> &ZonedTime {
        &self.zoned_time
    }
}

impl SetEqualTo for ZonedDateTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), crate::set_equal_to::SetEqualToError> {
        // Implement the logic to set this ZonedDateTime equal to another ZonedDateTime.
        todo!("Implement set_equal_to for ZonedDateTime");
    }
}

impl Accessor for ZonedDateTime {}
