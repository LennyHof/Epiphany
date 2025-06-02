use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for data specs.
#[derive(Debug, PartialEq)]
pub struct DataSpecSpec {}

impl PrimitiveSpec for DataSpecSpec {}

impl DataSpecSpec {
    /// Creates a new data spec spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this data spec spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all data specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for DataSpecSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataSpec")
    }
}