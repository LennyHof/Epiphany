use std::rc::Rc;

use crate::primitive_specs::float_spec::FloatSpec;

/// A number with a fractional part.
pub struct FloatPrimitive {
    /// The float's specification.
    pub spec: Rc<FloatSpec>,
    /// The float's storage.
    pub storage: Option<f64>,
}

impl FloatPrimitive {
    /// Returns an initialized FloatPrimitive without storage.
    pub fn new(sp: Rc<FloatSpec>) -> FloatPrimitive {
        FloatPrimitive {
            spec: sp.clone(),
            storage: None,
        }
    }

    /// Returns an initialized FloatPrimitive with storage.
    pub fn clone_with_storage(other: &FloatPrimitive) -> FloatPrimitive {
        FloatPrimitive {
            spec: other.spec.clone(),
            storage: Some(0.0f64),
        }
    }

    /// Returns the primitive's specification.
    pub fn spec(&self) -> &FloatSpec {
        &self.spec
    }

    /// Returns the primitive's storage.
    pub fn storage(&self) -> &Option<f64> {
        &self.storage
    }
}
