use crate::{
    adaptors::string_adaptors::utf32_string_adaptor::Utf32StringAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Utf32String provides access to Unicode UTF-32 encoded strings.
pub struct Utf32String {
    // The adaptor for the UTF-32 string.
    adaptor: Box<dyn Utf32StringAdaptor>,
}

impl Utf32String {
    /// Creates a new Utf32String accessor.
    pub fn new(adaptor: Box<dyn Utf32StringAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Utf32String {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Utf32String");
    }
}

impl Accessor for Utf32String {}
