use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for walks.
#[derive(Debug, PartialEq)]
pub struct WalkSpec {}

impl WalkSpec {
    /// Creates a new walk spec.
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for WalkSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all walk specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for WalkSpec {
    fn is_ordered(&self) -> bool {
        false // Walks are not ordered.
    }
}

impl PrimitiveSpec for WalkSpec {}

impl std::fmt::Display for WalkSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Walk")
    }
}
