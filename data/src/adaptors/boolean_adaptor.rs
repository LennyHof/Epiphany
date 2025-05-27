use crate::provider_error::ProviderError;

/// An adaptor for Booleans.
pub trait BooleanAdaptor {
    /// Returns the value of the Boolean.
    fn boolean(&self) -> Result<bool, ProviderError>;

    /// Sets the value of the Boolean.
    fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError>;
}
