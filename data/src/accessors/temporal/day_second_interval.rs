use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Accessor for day-second interval values.
pub struct DaySecondInterval {}

impl SetEqualTo for DaySecondInterval {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this DaySecondInterval equal to another DaySecondInterval.
        todo!("Implement set_equal_to for DaySecondInterval");
    }
}

impl Accessor for DaySecondInterval {}
