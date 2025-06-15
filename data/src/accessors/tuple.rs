use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for time values.
pub struct Tuple {}

impl SetEqualTo for Tuple {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Tuple");
    }
}

impl Accessor for Tuple {}
