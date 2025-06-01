use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for dates.
pub struct DateSpec {}

impl DateSpec {
    /// Returns an initialized Date spec.
    pub fn new() -> DateSpec {
        DateSpec {}
    }
    /// Returns if this Date spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all Date specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl PrimitiveSpec for DateSpec {}
