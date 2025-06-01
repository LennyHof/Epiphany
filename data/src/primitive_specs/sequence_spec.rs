use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for sequences.
pub struct SequenceSpec {}

impl PrimitiveSpec for SequenceSpec {}

impl SequenceSpec {
    /// Creates a new sequence spec.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this sequence spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all sequence specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
