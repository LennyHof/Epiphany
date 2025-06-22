use crate::{
    adaptors::string_adaptors::byte_string_adaptor::ByteStringAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// ByteString provides access to <i>byte strings</i>, which are strings of 8-bit
/// characters.
pub struct ByteString {
    // The adaptor for the byte string.
    adaptor: Box<dyn ByteStringAdaptor>,
}

impl ByteString {
    /// Creates a new ByteString accessor.
    pub fn new(adaptor: Box<dyn ByteStringAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for ByteString {
    fn set_equal_to(&mut self, _other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for ByteString");
    }
}

impl Accessor for ByteString {}
