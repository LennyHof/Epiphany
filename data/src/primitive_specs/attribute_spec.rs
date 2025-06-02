use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for attributes.
#[derive(Debug, PartialEq)]
pub struct AttributeSpec {}

impl PrimitiveSpec for AttributeSpec {}

impl AttributeSpec {
    /// Creates a new attribute spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this attribute spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all attribute specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for AttributeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Attribute")
    }
}
