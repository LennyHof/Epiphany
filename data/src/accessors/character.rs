use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// The accessor for characters.
pub struct Character {}

impl SetEqualTo for Character {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        // Implement the logic to set this Character equal to another Character.
        todo!("Implement set_equal_to for Character");
    }
}

impl Accessor for Character {}
