use std::sync::Arc;

use crate::{data_spec::DataSpec, primitive_def::PrimitiveSpec};

/// A primitive spec for sets.
pub struct SetSpec {
    element_spec: Option<Arc<DataSpec>>,
}

impl PrimitiveSpec for SetSpec {}

impl SetSpec {
    /// Returns an initialized set spec.
    /// Prefer to use the [`SetSpecBuilder`](crate::spec_builders::set_spec_builder::SetSpecBuilder) to create a set spec.
    pub fn new(element_spec: &Option<Arc<DataSpec>>) -> SetSpec {
        SetSpec {
            element_spec: (element_spec.clone()),
        }
    }

    /// Returns the set's element specification.
    /// If the set does not have an element specification, this will return None.
    /// If the set has an element specification, this will return Some(spec), where spec is the element specification.
    pub fn element_spec(&self) -> &Option<Arc<DataSpec>> {
        &self.element_spec
    }
}
impl Default for SetSpec {
    fn default() -> Self {
        Self::new(&None)
    }
}
