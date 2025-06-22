use crate::{
    adaptors::identifier_adaptor::IdentifierAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// The accessor for identifiers.
pub struct Identifier {
    adaptor: Box<dyn IdentifierAdaptor>,
}

impl Identifier {
    /// Creates a new `Identifier` accessor.
    pub fn new(adaptor: Box<dyn IdentifierAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Identifier {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Identifier");
    }
}

impl Accessor for Identifier {}
