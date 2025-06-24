use std::rc::Rc;

use crate::{
    data_spec::DataSpec,
    primitive_def::{IsOrdered, PrimitiveSpec},
    spec_compatibility::SpecCompatibility,
};

/// A primitive spec for tuples.
#[derive(Debug, PartialEq)]
pub struct TupleSpec {
    value_specs: Option<Vec<Rc<DataSpec>>>,
}

impl TupleSpec {
    /// Creates a new tuple spec.
    pub(crate) fn new(value_specs: Option<Vec<Rc<DataSpec>>>) -> Self {
        Self { value_specs }
    }

    /// Returns the value specs of the tuple.
    pub fn value_specs(&self) -> &Option<Vec<Rc<DataSpec>>> {
        &self.value_specs
    }

    /// Returns the tuple's length.
    pub fn len(&self) -> usize {
        self.value_specs.as_ref().map_or(0, |specs| specs.len())
    }
}

impl SpecCompatibility for TupleSpec {
    fn is_compatible_with(&self, _required: &Self) -> bool {
        match (self.value_specs.as_ref(), _required.value_specs.as_ref()) {
            (Some(_), Some(_)) => {
                if let (Some(value_specs), Some(required_value_specs)) =
                    (self.value_specs.as_ref(), _required.value_specs.as_ref())
                {
                    if value_specs.len() != required_value_specs.len() {
                        return false; // length mismatch
                    }
                    for (value_spec, required_value_spec) in
                        value_specs.iter().zip(required_value_specs)
                    {
                        if !value_spec.is_compatible_with(required_value_spec) {
                            return false; // incompatible value specs
                        }
                    }
                }
                true // all value specs are compatible
            }
            (None, None) => true,
            (Some(_), None) => true, // required does not specify value specs, so we assume compatibility
            (None, Some(_)) => false,
        }
    }
}

impl IsOrdered for TupleSpec {
    fn is_ordered(&self) -> bool {
        true // Tuples are considered ordered collections.
    }
}

impl PrimitiveSpec for TupleSpec {}

impl std::fmt::Display for TupleSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tuple {{ value_specs: {{")?;
        if self.value_specs.is_none() {
            return write!(f, " None }} }}");
        }
        let mut first = true;
        for value_spec in self.value_specs.as_ref().unwrap().iter() {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{}", value_spec)?;
        }
        write!(f, "}} }}")
    }
}
