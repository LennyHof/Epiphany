use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for edges.
#[derive(Debug, PartialEq)]
pub struct EdgeSpec {}

impl EdgeSpec {
    /// Creates a new edge spec.
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for EdgeSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all edge specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for EdgeSpec {
    fn is_ordered(&self) -> bool {
        false // Edges are not hashable.
    }
}

impl PrimitiveSpec for EdgeSpec {}

impl std::fmt::Display for EdgeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Edge")
    }
}
