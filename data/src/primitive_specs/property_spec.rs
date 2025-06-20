use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for properties.
#[derive(Debug, PartialEq)]
pub struct PropertySpec {}

impl PropertySpec {
    /// Creates a new property spec.
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for PropertySpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all property specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for PropertySpec {
    fn is_ordered(&self) -> bool {
        // Properties are ordered.
        true
    }
}

impl PrimitiveSpec for PropertySpec {}

impl std::fmt::Display for PropertySpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Property")
    }
}
