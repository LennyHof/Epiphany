use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for tuples.
#[derive(Debug, PartialEq)]
pub struct TupleSpec {}

impl PrimitiveSpec for TupleSpec {}

impl TupleSpec {
    /// Creates a new tuple spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this tuple spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all tuple specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl std::fmt::Display for TupleSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple")
    }
}
