use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for characters.
#[derive(Debug, PartialEq)]
pub struct CharacterSpec {}

impl CharacterSpec {
    /// Returns an initialized Character spec.
    pub fn new() -> CharacterSpec {
        CharacterSpec {}
    }
}

impl SpecCompatibility for CharacterSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all Character specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for CharacterSpec {
    fn is_ordered(&self) -> bool {
        true // Characters are hashable.
    }
}

impl PrimitiveSpec for CharacterSpec {}

impl std::fmt::Display for CharacterSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Character")
    }
}
