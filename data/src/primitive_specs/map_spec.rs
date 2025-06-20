use std::{fmt::Display, rc::Rc};

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// The ordering of keys in a map.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapKeyOrdering {
    /// An ordered map, where the order of keys matters.
    Ordered,
    /// An unordered map, where the order of keys does not matter.
    Unordered,
}

impl Display for MapKeyOrdering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Ordered => "Ordered".to_string(),
                Self::Unordered => "Unordered".to_string(),
            }
        )
    }
}

/// A primitive spec for maps.
#[derive(Debug, PartialEq)]
pub struct MapSpec {
    key_spec: Option<Rc<DataSpec>>,
    value_spec: Option<Rc<DataSpec>>,
    key_ordering: Option<MapKeyOrdering>,
}

impl MapSpec {
    /// Returns an initialized map spec.
    /// Prefer to use the [`MapSpecBuilder`](crate::data_spec_builders::map_spec_builder::MapSpecBuilder) to create a map spec.
    pub fn new(
        key_spec: &Option<Rc<DataSpec>>,
        value_spec: &Option<Rc<DataSpec>>,
        key_ordering: Option<MapKeyOrdering>,
    ) -> MapSpec {
        MapSpec {
            key_spec: (key_spec.clone()),
            value_spec: (value_spec.clone()),
            key_ordering: (key_ordering),
        }
    }

    /// Returns the map's key specification.
    /// If the map does not have a key specification, this will return None.
    /// If the map has a key specification, this will return Some(spec), where spec is the key specification.
    /// Returns the map's key specification.
    pub fn key_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.key_spec
    }

    /// Returns the map's value specification.
    /// If the map does not have an value specification, this will return None.
    /// If the map has an value specification, this will return Some(spec), where spec is the value specification.
    pub fn value_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.value_spec
    }

    /// Returns the map's key ordering.
    /// If the map does not have a key ordering specified, this will return None.
    pub fn key_ordering(&self) -> &Option<MapKeyOrdering> {
        &self.key_ordering
    }
}

impl SpecCompatibility for MapSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        if self.key_spec.is_some() && required.key_spec.is_some() {
            if let Some(key_spec) = self.key_spec.as_ref() {
                if let Some(required_key_spec) = required.key_spec.as_ref() {
                    if !key_spec.is_compatible_with(required_key_spec) {
                        return false;
                    }
                }
            }
        } else if self.key_spec.is_none() && required.key_spec.is_some() {
            return false;
        }

        if self.value_spec.is_some() && required.value_spec.is_some() {
            if let Some(value_spec) = self.value_spec.as_ref() {
                if let Some(required_value_spec) = required.value_spec.as_ref() {
                    if !value_spec.is_compatible_with(required_value_spec) {
                        return false;
                    }
                }
            }
        } else if self.value_spec.is_none() && required.value_spec.is_some() {
            return false;
        }

        match (self.key_ordering, required.key_ordering) {
            (Some(s), Some(r)) => s == r,
            (None, None) => true,
            (Some(_), None) => true, // required does not specify key_ordering, so we assume compatibility
            (None, Some(_)) => false,
        }
    }
}

impl PrimitiveSpec for MapSpec {}

impl IsOrdered for MapSpec {
    fn is_ordered(&self) -> bool {
        // A map is ordered if its keys are ordered and its element spec is ordered.
        // If key ordering or element spec is None, we assume it is not ordered.
        self.key_ordering.as_ref() == Some(&MapKeyOrdering::Ordered)
            && self.key_spec.as_ref().is_some_and(|k| k.is_ordered())
    }
}

impl std::fmt::Display for MapSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Map {{ key_spec: {}, value_spec: {}, key_ordering: {} }}",
            self.key_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.value_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.key_ordering
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
