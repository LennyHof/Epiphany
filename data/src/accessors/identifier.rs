use crate::{adaptors::identifier_adaptor::IdentifierAdaptor, primitive_def::Accessor};

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

impl Accessor for Identifier {}
