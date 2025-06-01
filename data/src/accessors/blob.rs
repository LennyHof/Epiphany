use crate::{adaptors::blob_adaptor::BlobAdaptor, primitive_def::Accessor};

/// An accessor for BLOBs.
pub struct Blob {
    // The adaptor for the BLOB.
    adaptor: Box<dyn BlobAdaptor>,
}

impl Accessor for Blob {}

impl Blob {
    /// Creates a new `Blob` accessor.
    pub fn new(adaptor: Box<dyn BlobAdaptor>) -> Self {
        Self { adaptor }
    }
}
