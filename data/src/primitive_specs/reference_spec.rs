use crate::{
    accessors::schema::class::Class,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for references.
#[derive(Debug, PartialEq)]
pub struct ReferenceSpec {
    referenced_class: Class,
}

impl ReferenceSpec {
    /// Returns an initialized reference spec.
    pub fn new(referenced_class: Class) -> ReferenceSpec {
        ReferenceSpec {
            referenced_class: (referenced_class),
        }
    }
    /// Returns the referenced class.
    pub fn referenced_class(&self) -> &Class {
        &self.referenced_class
    }
}

impl SpecCompatibility for ReferenceSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        self.referenced_class
            .is_compatible_with(&required.referenced_class)
    }
}

impl IsOrdered for ReferenceSpec {
    fn is_ordered(&self) -> bool {
        true // References are hashable.
    }
}

impl PrimitiveSpec for ReferenceSpec {}

impl std::fmt::Display for ReferenceSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reference {{ class: {} }}", self.referenced_class.name())
    }
}
