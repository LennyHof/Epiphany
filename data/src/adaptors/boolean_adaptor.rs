use std::rc::Rc;

use crate::{primitive_specs::boolean_spec::BooleanSpec, provider_error::ProviderError};

/// An adaptor for Booleans.
pub trait BooleanAdaptor {
    /// Returns the boolean's specification.
    fn spec(&self) -> &Rc<BooleanSpec>;

    /// Returns the value of the Boolean.
    fn boolean(&self) -> Result<bool, ProviderError>;

    /// Sets the value of the Boolean.
    fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError>;
}
