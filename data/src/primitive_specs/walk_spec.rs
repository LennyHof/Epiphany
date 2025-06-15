use crate::{primitive_def::PrimitiveSpec, spec_compatibility::SpecCompatibility};

/// A primitive spec for walks.
#[derive(Debug, PartialEq)]
pub struct WalkSpec {}

impl WalkSpec {
    /// Creates a new walk spec.
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for WalkSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        // For now, we assume all walk specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl PrimitiveSpec for WalkSpec {}

impl std::fmt::Display for WalkSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Walk")
    }
}
