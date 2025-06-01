use crate::{
    adaptors::string_adaptors::utf8_string_adaptor::Utf8StringAdaptor, primitive_def::Accessor,
};

/// Utf8String provides access to Unicode UTF-8 encoded strings.
pub struct Utf8String {
    // The adaptor for the UTF-8 string.
    adaptor: Box<dyn Utf8StringAdaptor>,
}

impl Accessor for Utf8String {}

impl Utf8String {
    /// Creates a new Utf8String accessor.
    pub fn new(adaptor: Box<dyn Utf8StringAdaptor>) -> Self {
        Self { adaptor }
    }
}
