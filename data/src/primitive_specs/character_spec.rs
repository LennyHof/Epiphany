use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for characters.
#[derive(Debug, PartialEq)]
pub struct CharacterSpec {}

impl CharacterSpec {
    /// Returns an initialized Character spec.
    pub fn new() -> CharacterSpec {
        CharacterSpec {}
    }

    /// Returns if this Character spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all Character specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl PrimitiveSpec for CharacterSpec {}

impl std::fmt::Display for CharacterSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Character")
    }
}
