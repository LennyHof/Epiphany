use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Accessor for interval values.
pub struct Interval {}

impl SetEqualTo for Interval {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Interval equal to another Interval.
        todo!("Implement set_equal_to for Interval");
    }
}

impl Accessor for Interval {}
