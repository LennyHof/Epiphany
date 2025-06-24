use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for GUIDs.
#[derive(Debug, PartialEq)]
pub struct GuidSpec {}

impl GuidSpec {
    /// Returns an initialized GUID spec.
    pub(crate) fn new() -> GuidSpec {
        GuidSpec {}
    }
}

impl SpecCompatibility for GuidSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all GUID specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for GuidSpec {
    fn is_ordered(&self) -> bool {
        true // GUIDs are ordered.
    }
}

impl PrimitiveSpec for GuidSpec {}

impl Default for GuidSpec {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for GuidSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Guid")
    }
}
