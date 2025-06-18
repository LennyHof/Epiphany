use std::rc::Rc;

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for maps.
#[derive(Debug, PartialEq)]
pub struct MapSpec {
    key_spec: Option<Rc<DataSpec>>,
    element_spec: Option<Rc<DataSpec>>,
}

impl MapSpec {
    /// Returns an initialized map spec.
    /// Prefer to use the [`MapSpecBuilder`](crate::data_spec_builders::map_spec_builder::MapSpecBuilder) to create a map spec.
    pub fn new(key_spec: &Option<Rc<DataSpec>>, element_spec: &Option<Rc<DataSpec>>) -> MapSpec {
        MapSpec {
            key_spec: (key_spec.clone()),
            element_spec: (element_spec.clone()),
        }
    }

    /// Returns the map's key specification.
    /// If the map does not have a key specification, this will return None.
    /// If the map has a key specification, this will return Some(spec), where spec is the key specification.
    /// Returns the map's key specification.
    pub fn key_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.key_spec
    }

    /// Returns the map's element specification.
    /// If the map does not have an element specification, this will return None.
    /// If the map has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.element_spec
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

impl PrimitiveSpec for MapSpec {}

impl IsOrdered for MapSpec {
    fn is_ordered(&self) -> bool {
        // Maps are hashable if key spec is hashable.
        self.key_spec.as_ref().map_or(true, |k| k.is_ordered())
    }
}

impl std::fmt::Display for MapSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Map {{ key_spec: {}, element_spec: {} }}",
            self.key_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.element_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
