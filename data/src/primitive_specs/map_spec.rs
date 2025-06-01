use std::sync::Arc;

use crate::{data_spec::DataSpec, primitive_def::PrimitiveSpec};

/// A primitive spec for maps.
pub struct MapSpec {
    key_spec: Option<Arc<DataSpec>>,
    element_spec: Option<Arc<DataSpec>>,
}

impl PrimitiveSpec for MapSpec {}

impl MapSpec {
    /// Returns an initialized map spec.
    /// Prefer to use the [`MapSpecBuilder`](crate::data_spec_builders::map_spec_builder::MapSpecBuilder) to create a map spec.
    pub fn new(key_spec: &Option<Arc<DataSpec>>, element_spec: &Option<Arc<DataSpec>>) -> MapSpec {
        MapSpec {
            key_spec: (key_spec.clone()),
            element_spec: (element_spec.clone()),
        }
    }

    /// Returns the map's key specification.
    /// If the map does not have a key specification, this will return None.
    /// If the map has a key specification, this will return Some(spec), where spec is the key specification.
    /// Returns the map's key specification.
    pub fn key_spec(&self) -> &Option<Arc<DataSpec>> {
        &self.key_spec
    }

    /// Returns the map's element specification.
    /// If the map does not have an element specification, this will return None.
    /// If the map has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Arc<DataSpec>> {
        &self.element_spec
    }
    /// Returns if this map spec is compatible with the required spec.
    pub fn is_compatible_with(&self, required: &Self) -> bool {
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

        if self.element_spec.is_some() && required.element_spec.is_some() {
            if let Some(element_spec) = self.element_spec.as_ref() {
                if let Some(required_element_spec) = required.element_spec.as_ref() {
                    if !element_spec.is_compatible_with(required_element_spec) {
                        return false;
                    }
                }
            }
        } else if self.element_spec.is_none() && required.element_spec.is_some() {
            return false;
        }

        true
    }
}
