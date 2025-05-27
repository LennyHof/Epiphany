use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for characters.
pub struct CharacterSpec {}

impl CharacterSpec {
    /// Returns an initialized Character spec.
    pub fn new() -> CharacterSpec {
        CharacterSpec {}
    }
}

impl PrimitiveSpec for CharacterSpec {}
