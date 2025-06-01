use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for properties.
pub struct PropertySpec {}

impl PrimitiveSpec for PropertySpec {}

impl PropertySpec {
    /// Creates a new property spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this property spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all property specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
