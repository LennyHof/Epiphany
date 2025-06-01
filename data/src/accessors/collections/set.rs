use crate::{adaptors::collection_adaptors::set_adaptor::SetAdaptor, primitive_def::Accessor};

/// The Set accessor provides access to sets, which are ordered or unordered collections of
/// unique elements.
pub struct Set {
    // The adaptor for the set.
    adaptor: Box<dyn SetAdaptor>,
}

impl Accessor for Set {}

impl Set {
    /// Creates a new `Set` accessor.
    pub fn new(adaptor: Box<dyn SetAdaptor>) -> Self {
        Self { adaptor }
    }
}
