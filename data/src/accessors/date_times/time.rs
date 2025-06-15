use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for time values.
pub struct Time {}

impl SetEqualTo for Time {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Time equal to another Time.
        todo!("Implement set_equal_to for Time");
    }
}

impl Accessor for Time {}
