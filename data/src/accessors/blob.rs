use crate::{
    adaptors::blob_adaptor::BlobAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// An accessor for BLOBs.
pub struct Blob {
    // The adaptor for the BLOB.
    adaptor: Box<dyn BlobAdaptor>,
}

impl Blob {
    /// Creates a new `Blob` accessor.
    pub fn new(adaptor: Box<dyn BlobAdaptor>) -> Self {
        Self { adaptor }
    }
}

impl SetEqualTo for Blob {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Blob");
    }
}

impl Accessor for Blob {}
