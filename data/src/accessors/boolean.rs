use crate::{
    adaptors::boolean_adaptor::BooleanAdaptor,
    data_provider::{DataProvider, default_data_provider},
    data_spec_builders::boolean_spec_builder::BooleanSpecBuilder,
    primitive_def::Accessor,
    provider_error::ProviderError,
    variable::Variable,
};

/// The accessor for booleans.
pub struct Boolean {
    adaptor: Box<dyn BooleanAdaptor>,
}

impl Boolean {
    /// Creates a new boolean accessor.
    pub fn new(adaptor: Box<dyn BooleanAdaptor>) -> Self {
        Self { adaptor }
    }

    /// Returns the value of the boolean.
    pub fn boolean(&self) -> Result<bool, ProviderError> {
        self.adaptor.boolean()
    }

    /// Sets the value of the boolean.
    pub fn set_boolean(&mut self, value: bool) -> Result<(), ProviderError> {
        self.adaptor.set_boolean(value)
    }
}

impl Accessor for Boolean {}

impl TryFrom<bool> for Variable {
    type Error = ProviderError;

    fn try_from(value: bool) -> Result<Self, Self::Error> {
        let spec = BooleanSpecBuilder::new().build();
        let mut var = default_data_provider().variable_for(&spec);
        let boolean = var.boolean_mut();
        boolean.set_boolean(value)?;
        Ok(var)
    }
}

impl TryFrom<Variable> for bool {
    type Error = ProviderError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let boolean = value.boolean();
        let result = boolean.boolean()?;
        Ok(result)
    }
}
