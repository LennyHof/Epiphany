use crate::variable::Variable;

/// A trait for adaptors that provide access to identifiers.
pub trait IdentifierAdaptor {
    /// Returns the identifier's specification.
    fn spec(&self) -> &str;

    /// Returns the identifier's value.
    fn value(&self) -> &Variable;

    /// Sets the identifier's value.
    fn set_value(&mut self, value: Variable) -> Result<(), String>;
}
