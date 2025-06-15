use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for objects.
pub struct Object {}

impl SetEqualTo for Object {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Object");
    }
}

impl Accessor for Object {}
