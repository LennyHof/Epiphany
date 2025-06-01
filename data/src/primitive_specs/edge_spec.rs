use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for edges.
pub struct EdgeSpec {}

impl PrimitiveSpec for EdgeSpec {}

impl EdgeSpec {
    /// Creates a new edge spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this edge spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all edge specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
