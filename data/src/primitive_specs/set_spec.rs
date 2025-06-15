use std::rc::Rc;

use crate::{
    data_spec::DataSpec, primitive_def::PrimitiveSpec, spec_compatibility::SpecCompatibility,
};

/// A primitive spec for sets.
#[derive(Debug, PartialEq)]
pub struct SetSpec {
    element_spec: Option<Rc<DataSpec>>,
}

impl SetSpec {
    /// Returns an initialized set spec.
    /// Prefer to use the [`SetSpecBuilder`](crate::data_spec_builders::set_spec_builder::SetSpecBuilder) to create a set spec.
    pub fn new(element_spec: &Option<Rc<DataSpec>>) -> SetSpec {
        SetSpec {
            element_spec: (element_spec.clone()),
        }
    }

    /// Returns the set's element specification.
    /// If the set does not have an element specification, this will return None.
    /// If the set has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Rc<DataSpec>> {
        &self.element_spec
    }
}

impl SpecCompatibility for SetSpec {
    fn is_compatible_with(&self, required: &Self) -> bool {
        if self.element_spec.is_some() && required.element_spec.is_some() {
            if let Some(element_spec) = self.element_spec.as_ref() {
                if let Some(required_element_spec) = required.element_spec.as_ref() {
                    return element_spec.is_compatible_with(required_element_spec);
                }
            }
        } else if self.element_spec.is_none() && required.element_spec.is_some() {
            return false;
        }
        true
    }
}

impl PrimitiveSpec for SetSpec {}

impl Default for SetSpec {
    fn default() -> Self {
        Self::new(&None)
    }
}

impl std::fmt::Display for SetSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Set {{ element_spec: {} }}",
            self.element_spec
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "None".to_string())
        )
    }
}
