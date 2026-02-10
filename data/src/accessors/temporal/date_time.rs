use crate::{
    accessors::temporal::{date::Date, time::Time},
    adaptors::temporal_adaptors::{date_adaptor::DateAdaptor, time_adaptor::TimeAdaptor},
    primitive_def::Accessor,
    set_equal_to::SetEqualTo,
};

/// Accessor for date-time values.
pub struct DateTime {
    date: Date,
    time: Time,
}

impl DateTime {
    /// Creates a new DateTime accessor.
    pub fn new(date_adaptor: Box<dyn DateAdaptor>, time_adaptor: Box<dyn TimeAdaptor>) -> Self {
        Self {
            date: Date::new(date_adaptor),
            time: Time::new(time_adaptor),
        }
    }

    /// Returns the date-time's date component.
    fn date(&self) -> &Date {
        &self.date
    }

    /// Returns the date-time's time component.
    pub fn time(&self) -> &Time {
        &self.time
    }
}

impl SetEqualTo for DateTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), crate::set_equal_to::SetEqualToError> {
        // Implement the logic to set this DateTime equal to another DateTime.
        todo!("Implement set_equal_to for DateTime");
    }
}

impl Accessor for DateTime {}
