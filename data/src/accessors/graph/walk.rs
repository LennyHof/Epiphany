use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for walks.
pub struct Walk {}

impl SetEqualTo for Walk {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Walk equal to another Walk.
        todo!("Implement set_equal_to for Walk");
    }
}

impl Accessor for Walk {}
