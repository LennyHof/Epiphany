use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for Booleans.
pub struct BooleanSpec {}

impl BooleanSpec {
    /// Returns an initialized Boolean spec.
    /// Prefer to use the [`BooleanSpecBuilder`](crate::spec_builders::boolean_spec_builder::BooleanSpecBuilder) to create a Boolean spec.
    pub fn new() -> BooleanSpec {
        BooleanSpec {}
    }
}

impl PrimitiveSpec for BooleanSpec {}

impl Default for BooleanSpec {
    fn default() -> Self {
        Self::new()
    }
}
