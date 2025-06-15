use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for attributes.
pub struct Attribute {}

impl SetEqualTo for Attribute {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Attribute equal to another Attribute.
        todo!("Implement set_equal_to for Attribute");
    }
}

impl Accessor for Attribute {}
