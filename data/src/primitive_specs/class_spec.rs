use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for classes.
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
