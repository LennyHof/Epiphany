use crate::{
    adaptors::collection_adaptors::map_adaptor::MapAdaptor,
    primitive_def::Accessor,
    set_equal_to::{SetEqualTo, SetEqualToError},
};

/// The Map accessor provides access to maps,
/// which are ordered or unordered collections of unique key values that lead to
/// potentially non-unique element values.
pub struct Map {
    // The adaptor for the map.
    adaptor: Box<dyn MapAdaptor>,
}

impl SetEqualTo for Map {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        todo!("Implement set_equal_to for Map");
    }
}

impl Accessor for Map {}

impl Map {
    /// Creates a new `Map` accessor.
    pub fn new(adaptor: Box<dyn MapAdaptor>) -> Self {
        Self { adaptor }
    }
}

/// Errors that can occur when working with maps.
#[derive(Debug, PartialEq)]
pub enum MapError {
    /// An error that occurs when the key is not found in the map.
    KeyNotFound(String),
    /// An error that occurs when the value cannot be set.
    ValueSetError(String),
}

impl From<MapError> for SetEqualToError {
    fn from(error: MapError) -> Self {
        SetEqualToError::MapError(error)
    }
}
