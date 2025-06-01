use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for Booleans.
pub struct BooleanSpec {}

impl BooleanSpec {
    /// Returns an initialized Boolean spec.
    /// Prefer to use the [`BooleanSpecBuilder`](crate::data_spec_builders::boolean_spec_builder::BooleanSpecBuilder) to create a Boolean spec.
    pub fn new() -> BooleanSpec {
        BooleanSpec {}
    }

    /// Returns if this Boolean spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all Boolean specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}

impl PrimitiveSpec for BooleanSpec {}

impl Default for BooleanSpec {
    fn default() -> Self {
        Self::new()
    }
}
