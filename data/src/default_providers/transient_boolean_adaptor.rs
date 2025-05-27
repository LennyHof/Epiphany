use crate::adaptors::boolean_adaptor::BooleanAdaptor;
use crate::provider_error::ProviderError;
pub struct TransientBooleanAdaptor {
    value: bool,
}

impl TransientBooleanAdaptor {
    /// Creates a new transient boolean adaptor.
    pub fn new() -> Self {
        Self { value: false }
    }
}

impl BooleanAdaptor for TransientBooleanAdaptor {
    fn boolean(&self) -> Result<bool, ProviderError> {
        Ok(self.value)
    }

    fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError> {
        self.value = value;
        Ok(())
    }
}
