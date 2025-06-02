use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for classes.
#[derive(Debug, PartialEq)]
pub struct ClassSpec {}

impl PrimitiveSpec for ClassSpec {}

impl ClassSpec {
    /// Creates a new class spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this class spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all class specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for ClassSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Class")
    }
}