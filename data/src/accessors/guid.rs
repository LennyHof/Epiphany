use crate::{adaptors::guid_adaptor::GuidAdaptor, primitive_def::Accessor};

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

impl Accessor for Guid {}
