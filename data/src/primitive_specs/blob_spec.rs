use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for BLOBs.
#[derive(Debug, PartialEq)]
pub struct BlobSpec {}

impl PrimitiveSpec for BlobSpec {}

impl BlobSpec {
    /// Creates a new BLOB spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this BLOB spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all BLOB specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for BlobSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BLOB")
    }
}
