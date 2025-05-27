use crate::accessors::schema::class::Class;

/// A spec for the object-or-reference primitive category.
pub struct ObjectOrReferenceSpec {
    associated_class: Class,
}

impl ObjectOrReferenceSpec {
    /// Returns an initialized object-or-reference spec.
    pub fn new(associated_class: Class) -> ObjectOrReferenceSpec {
        ObjectOrReferenceSpec {
            associated_class: (associated_class),
        }
    }
    /// Returns the associated class.
    pub fn associated_class(&self) -> &Class {
        &self.associated_class
    }
}
