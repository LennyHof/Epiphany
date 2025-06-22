use crate::{
    adaptors::guid_adaptor::GuidAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// The accessor for GUIDs.
pub struct Guid {
    adaptor: Box<dyn GuidAdaptor>,
}

impl Guid {
    /// Creates a new `Guid` accessor.
    pub fn new(adaptor: Box<dyn GuidAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Guid {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Guid");
    }
}

impl Accessor for Guid {}
