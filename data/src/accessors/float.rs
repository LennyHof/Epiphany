use std::{error::Error, fmt::Display, sync::Arc};

use crate::{
    adaptors::float_adaptor::FloatAdaptor,
    data_provider::{DataProvider, default_data_provider},
    primitive_def::Accessor,
    primitive_specs::float_spec::{FloatSpec, FloatStorage},
    provider_error::ProviderError,
    spec_builders::float_spec_builder::FloatSpecBuilder,
    variable::Variable,
};

/// An accessor for float values.
pub struct Float {
    adaptor: Box<dyn FloatAdaptor>,
}

impl Float {
    /// Returns a new Float that uses the provided adaptor.
    pub fn new(adaptor: Box<dyn FloatAdaptor>) -> Float {
        Float { adaptor }
    }

    /// Returns the float's specification
    pub fn spec(&self) -> &Arc<FloatSpec> {
        self.adaptor.spec()
    }

    /// Sets the float value as a 64 bit value.
    pub fn set_f64(&mut self, value: f64) -> Result<(), FloatError> {
        self.adaptor.set_f64(value)
    }

    /// Returns the float value as a 64 bit value.
    pub fn f64(&self) -> Result<f64, FloatError> {
        self.adaptor.f64()
    }
}

impl Accessor for Float {}

// Convert all float types to a Variable
impl TryFrom<f32> for Variable {
    type Error = FloatError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let spec = FloatSpecBuilder::new()
            .set_storage(FloatStorage::B32)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let float = var.float_mut();
        float.set_f64(value as f64)?;
        Ok(var)
    }
}
impl TryFrom<f64> for Variable {
    type Error = FloatError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let spec = FloatSpecBuilder::new()
            .set_storage(FloatStorage::B64)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let float = var.float_mut();
        float.set_f64(value)?;
        Ok(var)
    }
}
/// Convert all Variable types to a float
impl TryFrom<Variable> for f32 {
    type Error = FloatError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let float = value.float();
        let result = float.f64()?;
        Ok(result as f32)
    }
}
impl TryFrom<Variable> for f64 {
    type Error = FloatError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let float = value.float();
        let result = float.f64()?;
        Ok(result)
    }
}
/// An float error.
#[derive(Debug, PartialEq)]
pub enum FloatError {
    /// A provider error
    ProviderError(ProviderError),
    /// An overflow error
    Overflow(String),
}

impl Error for FloatError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FloatError::ProviderError(e) => Some(e),
            FloatError::Overflow(_) => None,
        }
    }
}

impl Display for FloatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FloatError::ProviderError(err) => write!(f, "{:?}", err),
            FloatError::Overflow(msg) => write!(f, "Overflow Error: {}", msg),
        }
    }
}

impl From<String> for FloatError {
    fn from(value: String) -> Self {
        FloatError::ProviderError(ProviderError::General(value))
    }
}

impl From<ProviderError> for FloatError {
    fn from(value: ProviderError) -> Self {
        FloatError::ProviderError(value)
    }
}
