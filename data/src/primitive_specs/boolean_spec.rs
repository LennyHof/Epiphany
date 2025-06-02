use std::fmt::Display;

use crate::primitive_def::PrimitiveSpec;

/// A primitive spec for Booleans.
#[derive(Debug, PartialEq)]
pub struct BooleanSpec {}

impl BooleanSpec {
    /// Returns an initialized Boolean spec.
    /// Prefer to use the [`BooleanSpecBuilder`](crate::data_spec_builders::boolean_spec_builder::BooleanSpecBuilder) to create a Boolean spec.
    pub fn new() -> BooleanSpec {
        BooleanSpec {}
    }

    /// Returns if this Boolean spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        true // Boolean specs don't have any specializations.
    }
}

impl PrimitiveSpec for BooleanSpec {}

impl Display for BooleanSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boolean")
    }
}

impl Default for BooleanSpec {
    fn default() -> Self {
        Self::new()
    }
}
