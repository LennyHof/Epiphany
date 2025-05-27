use crate::primitive_def::Accessor;

/// Class provides access to schema classes.
pub struct Class {
    name: String,
}

impl Class {
    /// Returns the class's name.
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Accessor for Class {}
