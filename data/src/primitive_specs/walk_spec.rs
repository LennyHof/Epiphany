use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for walks.
#[derive(Debug, PartialEq)]
pub struct WalkSpec {}

impl PrimitiveSpec for WalkSpec {}

impl WalkSpec {
    /// Creates a new walk spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this walk spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all walk specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for WalkSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Walk")
    }
}
