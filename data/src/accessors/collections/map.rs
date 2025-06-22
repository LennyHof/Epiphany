use std::{fmt::Debug, fmt::Display, hash::Hash, rc::Rc};

use crate::{
    adaptors::collection_adaptors::map_adaptor::MapAdaptor,
    primitive_def::Accessor,
    primitive_specs::map_spec::{MapKeyOrdering, MapSpec},
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::{SpecCompatibility, SpecError},
    variable::Variable,
};

/// The Map accessor provides access to maps,
/// which are ordered or unordered collections of unique key values that lead to
/// potentially non-unique element values.
pub struct Map {
    // The adaptor for the map.
    adaptor: Box<dyn MapAdaptor>,
}

impl Map {
    /// Creates a new `Map` accessor.
    pub fn new(adaptor: Box<dyn MapAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the map's specification.
    pub fn spec(&self) -> &Rc<MapSpec> {
        self.adaptor.spec()
    }

    /// Checks if the map is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the map.
    pub fn len(&self) -> usize {
        self.adaptor.len()
    }

    /// Checks if the map contains a key.
    pub fn contains(&self, key: &Variable) -> Result<bool, MapError> {
        self.adaptor.contains(key)
    }

    /// Adds a key-value pair to the map. Returns `true` if the key was added, `false` if the key was already present.
    /// If the key already exists, the value is updated.
    ///
    /// This method checks if the key's data specification is compatible with the map's key specification
    /// and if the value's data specification is compatible with the map's value specification.
    pub fn insert(&mut self, key: Variable, value: Variable) -> Result<bool, MapError> {
        self.adaptor.insert(key, value)
    }

    /// Gets a value from the map by its key.
    pub fn get(&self, key: &Variable) -> Result<Option<Variable>, MapError> {
        self.adaptor.get(key)
    }

    /// Gets a mutable reference to a value in the map by its key.
    pub fn get_mut(&mut self, key: &Variable) -> Result<Option<&mut Variable>, MapError> {
        self.adaptor.get_mut(key)
    }

    /// Removes a key from the map.
    pub fn remove(&mut self, key: &Variable) -> Result<bool, MapError> {
        self.adaptor.remove(key)
    }

    /// Clears the map.
    pub fn clear(&mut self) -> Result<(), MapError> {
        self.adaptor.clear()
    }

    /// Returns an iterator for the map's key/value pairs.
    pub fn iter<'a>(&'a self) -> Box<dyn MapIter<'a> + 'a> {
        self.adaptor.iter()
    }

    /// Returns an iterator for the map's key/value pairs where the value is mutable.
    pub fn iter_mut<'a>(&'a mut self) -> Box<dyn MapIterMut<'a> + 'a> {
        self.adaptor.iter_mut()
    }

    /// Returns an iterator for the map's keys.
    pub fn keys<'a>(&'a self) -> Box<dyn MapKeysIter<'a> + 'a> {
        self.adaptor.keys()
    }

    /// Returns an iterator for the map's values.
    pub fn values<'a>(&'a self) -> Box<dyn MapValuesIter<'a> + 'a> {
        self.adaptor.values()
    }

    /// Returns an iterator for mutable map values.
    pub fn values_mut<'a>(&'a mut self) -> Box<dyn MapValuesIterMut<'a> + 'a> {
        self.adaptor.values_mut()
    }
}

/// An iterator for map key/value pairs.
pub trait MapIter<'a>: Iterator<Item = Result<(&'a Variable, &'a Variable), MapError>> {}

/// An iterator for map key/value pairs where the value is mutable.
pub trait MapIterMut<'a>:
    Iterator<Item = Result<(&'a Variable, &'a mut Variable), MapError>>
{
}

/// An iterator for map keys.
pub trait MapKeysIter<'a>: Iterator<Item = Result<&'a Variable, MapError>> {}

/// An iterator for map values.
pub trait MapValuesIter<'a>: Iterator<Item = Result<&'a Variable, MapError>> {}

/// An iterator for mutable map values.
pub trait MapValuesIterMut<'a>: Iterator<Item = Result<&'a mut Variable, MapError>> {}

impl<'a> IntoIterator for &'a Map {
    type Item = Result<(&'a Variable, &'a Variable), MapError>;
    type IntoIter = Box<dyn MapIter<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Map {
    type Item = Result<(&'a Variable, &'a mut Variable), MapError>;
    type IntoIter = Box<dyn MapIterMut<'a> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl SetEqualTo for Map {
    fn set_equal_to(&mut self, other: &Self) -> Result<(), SetEqualToError> {
        // Check if the map's specification is compatible with the other map's specification.
        self.spec().as_ref().check_compatible_with(other.spec())?;

        // Clear the current map.
        self.clear()?;

        // Iterate over the keys of the other map and insert cloned key-values into this map.
        for key_result in other.keys() {
            match key_result {
                Ok(key) => {
                    let cloned_key = key.try_clone()?;
                    let value = other.get(key)?;
                    // Clone the value to ensure it can be inserted into this map.
                    let cloned_value = value.unwrap().try_clone()?;
                    self.insert(cloned_key, cloned_value)?;
                }
                Err(e) => return Err(SetEqualToError::from(e)),
            }
        }
        Ok(())
    }
}

impl Accessor for Map {}

impl PartialOrd for Map {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Map {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match *self.spec().as_ref().key_ordering() {
            Some(MapKeyOrdering::Ordered) => {
                // Ordered maps can be compared based on their keys.
                for key_result in self.keys() {
                    match key_result {
                        Ok(key) => {
                            let other_value = other.get(key).ok().flatten();
                            if other_value != self.get(key).ok().flatten() {
                                return std::cmp::Ordering::Less;
                            }
                        }
                        Err(_) => return std::cmp::Ordering::Less,
                    }
                }
                std::cmp::Ordering::Equal
            }
            _ => {
                // If no key ordering is specified, we treat it as unordered.
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for key_result in self.keys() {
            match key_result {
                Ok(key) => {
                    let other_value = other.get(key).ok().flatten();
                    if other_value != self.get(key).ok().flatten() {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        }
        true
    }
}

impl Eq for Map {}

impl Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.spec().as_ref().key_ordering() {
            Some(MapKeyOrdering::Ordered) => {
                // Hash the map's keys and values.
                for key_result in self.keys() {
                    match key_result {
                        Ok(key) => {
                            key.hash(state);
                            if let Some(value) = self.get(key).ok().flatten() {
                                value.hash(state);
                            }
                        }
                        Err(_) => continue, // Ignore errors in hashing
                    }
                }
            }
            _ => {
                // If no key ordering is specified, we treat it as unordered.
                // This is not a valid operation for unordered maps, so we panic.
                panic!("Hashing of unordered maps is not supported.");
            }
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Map {{")?;
        let mut first = true;
        for result in self.iter() {
            match result {
                Ok((key, value)) => {
                    if !first {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                    first = false;
                }
                Err(e) => write!(f, "<error: {}>", e)?,
            }
        }
        write!(f, "}}")
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

/// Errors that can occur when working with maps.
#[derive(Debug, PartialEq)]
pub enum MapError {
    /// An error indicating that the key specification is incompatible with the map's key specification.
    KeySpecError(SpecError),
    /// An error indicating that the value specification is incompatible with the map's value specification.
    ValueSpecError(SpecError),
}

impl From<MapError> for SetEqualToError {
    fn from(error: MapError) -> Self {
        SetEqualToError::MapError(error)
    }
}

impl Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::KeySpecError(e) => write!(f, "Key specification error: {}", e),
            MapError::ValueSpecError(e) => write!(f, "Value specification error: {}", e),
        }
    }
}

impl std::error::Error for MapError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MapError::KeySpecError(e) => Some(e),
            MapError::ValueSpecError(e) => Some(e),
        }
    }
}
