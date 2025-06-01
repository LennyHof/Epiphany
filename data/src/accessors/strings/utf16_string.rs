use crate::{
    adaptors::string_adaptors::utf16_string_adaptor::Utf16StringAdaptor, primitive_def::Accessor,
};

/// Utf16String provides access to Unicode UTF-16 encoded strings.
pub struct Utf16String {
    // The adaptor for the UTF-16 string.
    adaptor: Box<dyn Utf16StringAdaptor>,
}

impl Accessor for Utf16String {}

impl Utf16String {
    /// Creates a new Utf16String accessor.
    pub fn new(adaptor: Box<dyn Utf16StringAdaptor>) -> Self {
        Self { adaptor }
    }
}
