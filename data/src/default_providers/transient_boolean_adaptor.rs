use std::rc::Rc;

use crate::adaptors::boolean_adaptor::BooleanAdaptor;
use crate::primitive_specs::boolean_spec::BooleanSpec;
use crate::provider_error::ProviderError;
pub struct TransientBooleanAdaptor {
    spec: Rc<BooleanSpec>,
    value: bool,
}

impl TransientBooleanAdaptor {
    /// Creates a new transient boolean adaptor.
    pub fn new() -> Self {
        Self {
            spec: Rc::new(BooleanSpec::new()),
            value: false,
        }
    }
}

impl BooleanAdaptor for TransientBooleanAdaptor {
    fn spec(&self) -> &Rc<BooleanSpec> {
        &self.spec
    }

    fn boolean(&self) -> Result<bool, ProviderError> {
        Ok(self.value)
    }

    fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError> {
        self.value = value;
        Ok(())
    }
}
