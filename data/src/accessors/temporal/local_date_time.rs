use crate::{primitive_def::Accessor, set_equal_to::SetEqualTo};

/// Accessor for date-time values.
pub struct LocalDateTime {}

impl SetEqualTo for LocalDateTime {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), crate::set_equal_to::SetEqualToError> {
        // Implement the logic to set this LocalDateTime equal to another LocalDateTime.
        todo!("Implement set_equal_to for LocalDateTime");
    }
}

impl Accessor for LocalDateTime {}
