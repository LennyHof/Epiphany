use crate::{
    adaptors::string_adaptors::utf8_string_adaptor::Utf8StringAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// Utf8String provides access to Unicode UTF-8 encoded strings.
pub struct Utf8String {
    // The adaptor for the UTF-8 string.
    adaptor: Box<dyn Utf8StringAdaptor>,
}

impl Utf8String {
    /// Creates a new Utf8String accessor.
    pub fn new(adaptor: Box<dyn Utf8StringAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Utf8String {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Utf8String");
    }
}

impl Accessor for Utf8String {}
