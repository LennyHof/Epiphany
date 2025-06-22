use std::rc::Rc;

use crate::{
    accessors::collections::map::{
        MapError, MapIter, MapIterMut, MapKeysIter, MapValuesIter, MapValuesIterMut,
    },
    primitive_specs::map_spec::MapSpec,
    spec_compatibility::SpecCompatibility,
    variable::Variable,
};

/// An adaptor for maps.
pub trait MapAdaptor {
    /// Returns the set's specification.
    fn spec(&self) -> &Rc<MapSpec>;

    /// Returns the length of the set.
    fn len(&self) -> usize;

    /// Returns whether the set is empty.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Checks if the map contains a key.
    /// Returns `true` if the key is in the map, `false` otherwise.
    fn contains(&self, key: &Variable) -> Result<bool, MapError>;

    /// Adds a key-value pair to the map. Returns `true` if the pair was added, `false` if the key was already present.
    fn insert(&mut self, key: Variable, value: Variable) -> Result<bool, MapError> {
        // Check if the key's data specification is compatible with the map's key specification.
        match key
            .data_spec()
            .check_compatible_with(self.spec().key_spec().as_ref().unwrap().as_ref())
        {
            Ok(_) => {}
            Err(e) => return Err(MapError::KeySpecError(e)),
        }
        // Check if the value's data specification is compatible with the map's value specification.
        match value
            .data_spec()
            .check_compatible_with(self.spec().value_spec().as_ref().unwrap().as_ref())
        {
            Ok(_) => {}
            Err(e) => return Err(MapError::ValueSpecError(e)),
        }
        self.do_insert(key, value)
    }

    /// Adds a key-value pair to the map. Returns `true` if the pair was added, `false` if the key was already present.
    fn do_insert(&mut self, key: Variable, value: Variable) -> Result<bool, MapError>;

    /// Gets a value from the map by its key.
    fn get(&self, key: &Variable) -> Result<Option<Variable>, MapError>;

    /// Gets a mutable reference to a value in the map by its key.
    fn get_mut(&mut self, key: &Variable) -> Result<Option<&mut Variable>, MapError>;

    /// Removes a key from the map.
    /// This method checks if the key's data specification is compatible with the map's key specification.
    /// If the key's data specification is not compatible, it returns a `MapError::KeySpecError`.
    fn remove(&mut self, key: &Variable) -> Result<bool, MapError> {
        // Check if the key's data specification is compatible with the map's key specification.
        match key
            .data_spec()
            .check_compatible_with(self.spec().key_spec().as_ref().unwrap().as_ref())
        {
            Ok(_) => {}
            Err(e) => return Err(MapError::KeySpecError(e)),
        }
        self.do_remove(key)
    }

    /// Removes a key from the map.
    fn do_remove(&mut self, key: &Variable) -> Result<bool, MapError>;

    /// Clears the map.
    fn clear(&mut self) -> Result<(), MapError>;

    /// Returns an iterator for the map's key/value pairs.
    fn iter<'a>(&'a self) -> Box<dyn MapIter<'a> + 'a>;

    /// Returns an iterator for the map's key/value pairs where the value is mutable.
    fn iter_mut<'a>(&'a mut self) -> Box<dyn MapIterMut<'a> + 'a>;

    /// Returns an iterator for the map's keys.
    fn keys<'a>(&'a self) -> Box<dyn MapKeysIter<'a> + 'a>;

    /// Returns an iterator for the map's values.
    fn values<'a>(&'a self) -> Box<dyn MapValuesIter<'a> + 'a>;

    /// Returns an iterator for the map's values as mutable.
    fn values_mut<'a>(&'a mut self) -> Box<dyn MapValuesIterMut<'a> + 'a>;
}
