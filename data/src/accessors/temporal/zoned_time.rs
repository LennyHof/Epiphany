use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for time values.
pub struct ZonedTime {}

impl SetEqualTo for ZonedTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this ZonedTime equal to another ZonedTime.
        todo!("Implement set_equal_to for ZonedTime");
    }
}

impl Accessor for ZonedTime {}
