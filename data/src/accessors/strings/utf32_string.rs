use crate::{
    adaptors::string_adaptors::utf32_string_adaptor::Utf32StringAdaptor, primitive_def::Accessor,
};

/// Utf32String provides access to Unicode UTF-32 encoded strings.
pub struct Utf32String {
    // The adaptor for the UTF-32 string.
    adaptor: Box<dyn Utf32StringAdaptor>,
}

impl Accessor for Utf32String {}

impl Utf32String {
    /// Creates a new Utf32String accessor.
    pub fn new(adaptor: Box<dyn Utf32StringAdaptor>) -> Self {
        Self { adaptor }
    }
}
