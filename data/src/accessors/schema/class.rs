use crate::primitive_def::Accessor;

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

    /// Returns if this class accessor is compatible with the required class accessor.
    pub fn is_compatible_with(&self, required: &Self) -> bool {
        // For now, we assume all class accessors are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        self.name == required.name
    }
}

impl Accessor for Class {}
