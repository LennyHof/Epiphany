use std::rc::Rc;

use crate::{
    adaptors::temporal_adaptors::day_to_second_duration_adaptor::DayToSecondDurationAdaptor,
    primitive_def::Accessor,
    primitive_specs::duration_spec::DurationSpec,
    provider_error::ProviderError,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Accessor for day-second duration values.
pub struct DayToSecondDuration {
    adaptor: Box<dyn DayToSecondDurationAdaptor>,
}

impl DayToSecondDuration {
    /// Creates a new DayToSecondDuration accessor.
    pub fn new(adaptor: Box<dyn DayToSecondDurationAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the day-second duration's specification.
    pub fn spec(&self) -> &Rc<DurationSpec> {
        self.adaptor.spec()
    }
}

impl SetEqualTo for DayToSecondDuration {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this DayToSecondDuration equal to another DayToSecondDuration.
        todo!("Implement set_equal_to for DayToSecondDuration");
    }
}

impl Accessor for DayToSecondDuration {}

/// Errors that can occur when working with DayToSecondDuration accessors.
pub enum DayToSecondDurationError {
    /// A provider error.
    ProviderError(ProviderError),
}

impl From<ProviderError> for DayToSecondDurationError {
    fn from(error: ProviderError) -> Self {
        DayToSecondDurationError::ProviderError(error)
    }
}
