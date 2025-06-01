use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for intervals.
pub struct IntervalSpec {}

impl PrimitiveSpec for IntervalSpec {}

impl IntervalSpec {
    /// Creates a new interval spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this interval spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all interval specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
