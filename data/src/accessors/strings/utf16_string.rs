use crate::{
    adaptors::string_adaptors::utf16_string_adaptor::Utf16StringAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Utf16String provides access to Unicode UTF-16 encoded strings.
pub struct Utf16String {
    // The adaptor for the UTF-16 string.
    adaptor: Box<dyn Utf16StringAdaptor>,
}

impl Utf16String {
    /// Creates a new Utf16String accessor.
    pub fn new(adaptor: Box<dyn Utf16StringAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Utf16String {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Utf16String");
    }
}

impl Accessor for Utf16String {}
