use crate::{
    accessors::schema::class::Class,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for objects.
#[derive(Debug, PartialEq)]
pub struct ObjectSpec {
    object_class: Class,
}

impl ObjectSpec {
    /// Returns an initialized object spec.
    pub(crate) fn new(object_class: Class) -> ObjectSpec {
        ObjectSpec {
            object_class: (object_class),
        }
    }
    /// Returns the object's class.
    pub fn object_class(&self) -> &Class {
        &self.object_class
    }
}

impl SpecCompatibility for ObjectSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        self.object_class.is_compatible_with(&required.object_class)
    }
}

impl IsOrdered for ObjectSpec {
    fn is_ordered(&self) -> bool {
        todo!("Implement hashability for ObjectSpec");
    }
}

impl PrimitiveSpec for ObjectSpec {}

impl std::fmt::Display for ObjectSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object {{class: {} }}", self.object_class.name())
    }
}
