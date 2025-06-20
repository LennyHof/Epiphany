use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for BLOBs.
#[derive(Debug, PartialEq)]
pub struct BlobSpec {}

impl BlobSpec {
    /// Creates a new BLOB spec.
    pub fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for BlobSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all BLOB specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for BlobSpec {
    fn is_ordered(&self) -> bool {
        // BLOBs are not ordered by default.
        false
    }
}

impl PrimitiveSpec for BlobSpec {}

impl std::fmt::Display for BlobSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BLOB")
    }
}
