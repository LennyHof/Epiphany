use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// EnumClass is a type of named value map that is defined and populated in schema. Each
/// member of an enumeration class is a named value.
pub struct EnumClass {}

impl SetEqualTo for EnumClass {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this EnumClass equal to another EnumClass.
        todo!("Implement set_equal_to for EnumClass");
    }
}

impl Accessor for EnumClass {}
