use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for intervals.
#[derive(Debug, PartialEq)]
pub struct IntervalSpec {}

impl IntervalSpec {
    /// Creates a new interval spec.
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for IntervalSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all interval specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for IntervalSpec {
    fn is_ordered(&self) -> bool {
        true // Intervals are hashable.
    }
}

impl PrimitiveSpec for IntervalSpec {}

impl std::fmt::Display for IntervalSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interval")
    }
}
