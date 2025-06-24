use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for attributes.
#[derive(Debug, PartialEq)]
pub struct AttributeSpec {}

impl AttributeSpec {
    /// Creates a new attribute spec.
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl std::fmt::Display for AttributeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attribute")
    }
}

impl SpecCompatibility for AttributeSpec {
    /// Checks if this attribute spec is compatible with the required spec.
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all attribute specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for AttributeSpec {
    fn is_ordered(&self) -> bool {
        true // Attributes are ordered.
    }
}

impl PrimitiveSpec for AttributeSpec {}
