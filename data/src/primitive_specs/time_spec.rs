use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for times.
#[derive(Debug, PartialEq)]
pub struct TimeSpec {}

impl TimeSpec {
    /// Returns an initialized Time spec.
    pub fn new() -> TimeSpec {
        TimeSpec {}
    }

    /// Returns if this time spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all time specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl PrimitiveSpec for TimeSpec {}

impl std::fmt::Display for TimeSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time")
    }
}
