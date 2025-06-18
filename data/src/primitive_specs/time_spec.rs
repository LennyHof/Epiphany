use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for times.
#[derive(Debug, PartialEq)]
pub struct TimeSpec {}

impl TimeSpec {
    /// Returns an initialized Time spec.
    pub fn new() -> TimeSpec {
        TimeSpec {}
    }
}

impl SpecCompatibility for TimeSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all time specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for TimeSpec {
    fn is_ordered(&self) -> bool {
        true // Times are hashable.
    }
}

impl PrimitiveSpec for TimeSpec {}

impl std::fmt::Display for TimeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time")
    }
}
