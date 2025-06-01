use crate::{adaptors::collection_adaptors::map_adaptor::MapAdaptor, primitive_def::Accessor};

/// The Map accessor provides access to maps,
/// which are ordered or unordered collections of unique key values that lead to
/// potentially non-unique element values.
pub struct Map {
    // The adaptor for the map.
    adaptor: Box<dyn MapAdaptor>,
}

impl Accessor for Map {}

impl Map {
    /// Creates a new `Map` accessor.
    pub fn new(adaptor: Box<dyn MapAdaptor>) -> Self {
        Self { adaptor }
    }
}
