use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for identifiers.
#[derive(Debug, PartialEq)]
pub struct IdentifierSpec {}

impl PrimitiveSpec for IdentifierSpec {}

impl IdentifierSpec {
    /// Creates a new identifier spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this identifier spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all identifier specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for IdentifierSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier")
    }
}