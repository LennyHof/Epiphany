use crate::{primitive_def::PrimitiveSpec, spec_compatibility::SpecCompatibility};

/// A primitive spec for GUIDs.
#[derive(Debug, PartialEq)]
pub struct GuidSpec {}

impl GuidSpec {
    /// Returns an initialized GUID spec.
    pub fn new() -> GuidSpec {
        GuidSpec {}
    }
}

impl SpecCompatibility for GuidSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        // For now, we assume all GUID specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl Default for GuidSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl PrimitiveSpec for GuidSpec {}

impl std::fmt::Display for GuidSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guid")
    }
}
