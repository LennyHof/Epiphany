use crate::{
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Class provides access to schema classes.
#[derive(Debug, PartialEq)]
pub struct Class {
    name: String,
}

impl Class {
    /// Creates a new class accessor.
    pub fn new(name: String) -> Class {
        Class { name }
    }

    /// Returns the class's name.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns if this class is compatible with required class.
    pub fn is_compatible_with(&self, required: &Self) -> bool {
        self.name == required.name
    }
}

impl SetEqualTo for Class {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Class");
    }
}

impl Accessor for Class {}
