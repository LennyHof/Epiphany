use crate::{accessors::schema::class::Class, primitive_def::PrimitiveSpec};

/// A primitive spec for references.
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

impl PrimitiveSpec for ReferenceSpec {}