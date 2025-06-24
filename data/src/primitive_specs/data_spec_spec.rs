use crate::{
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for data specs.
#[derive(Debug, PartialEq)]
pub struct DataSpecSpec {}

impl DataSpecSpec {
    /// Creates a new data spec spec.
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl SpecCompatibility for DataSpecSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all data specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl IsOrdered for DataSpecSpec {
    fn is_ordered(&self) -> bool {
        true // Data specs are ordered.
    }
}

impl PrimitiveSpec for DataSpecSpec {}

impl std::fmt::Display for DataSpecSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataSpec")
    }
}
