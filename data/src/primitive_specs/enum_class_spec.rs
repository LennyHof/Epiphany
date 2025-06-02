use crate::primitive_def::PrimitiveSpec;

/// The `EnumClassSpec` represents a specification for an enum class.
#[derive(Debug, PartialEq)]
pub struct EnumClassSpec {}
impl EnumClassSpec {
    /// Creates a new `EnumClassSpec`.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns if this enum class spec is compatible with the required spec.
    pub fn is_compatible_with(&self, _required: &Self) -> bool {
        // For now, we assume all enum class specs are compatible with each other.
        // This can be extended later to check specific compatibility rules.
        true
    }
}
impl PrimitiveSpec for EnumClassSpec {}

impl std::fmt::Display for EnumClassSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnumClass")
    }
}