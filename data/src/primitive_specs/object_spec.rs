use crate::{accessors::schema::class::Class, primitive_def::PrimitiveSpec};

/// A primitive spec for objects.
#[derive(Debug, PartialEq)]
pub struct ObjectSpec {
    object_class: Class,
}

impl ObjectSpec {
    /// Returns an initialized object spec.
    pub fn new(object_class: Class) -> ObjectSpec {
        ObjectSpec {
            object_class: (object_class),
        }
    }
    /// Returns the object's class.
    pub fn object_class(&self) -> &Class {
        &self.object_class
    }

    /// Returns if this object spec is compatible with the required spec.
    pub fn is_compatible_with(&self, required: &Self) -> bool {
        self.object_class.is_compatible_with(&required.object_class)
    }
}

impl PrimitiveSpec for ObjectSpec {}

impl std::fmt::Display for ObjectSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object {{class: {} }}", self.object_class.name())
    }
}
