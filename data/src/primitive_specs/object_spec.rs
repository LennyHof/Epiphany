use crate::{accessors::schema::class::Class, primitive_def::PrimitiveSpec};

/// A primitive spec for objects.
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
}

impl PrimitiveSpec for ObjectSpec {}