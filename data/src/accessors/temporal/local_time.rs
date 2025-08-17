use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for local time of day values.
pub struct LocalTime {}

impl SetEqualTo for LocalTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this LocalTime equal to another LocalTime.
        todo!("Implement set_equal_to for LocalTime");
    }
}

impl Accessor for LocalTime {}
