use std::{error::Error, fmt::Display, sync::Arc};

use crate::{
    adaptors::integer_adaptor::IntegerAdaptor,
    data_provider::{DataProvider, default_data_provider},
    data_spec_builders::integer_spec_builder::IntegerSpecBuilder,
    primitive_def::Accessor,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerSpec, IntegerStorage},
    provider_error::ProviderError,
    variable::Variable,
};

/// The accessor for integers.
pub struct Integer {
    adaptor: Box<dyn IntegerAdaptor>,
}

impl Integer {
    /// Returns a new Integer that uses the provided adaptor.
    pub fn new(adaptor: Box<dyn IntegerAdaptor>) -> Integer {
        Integer { adaptor }
    }

    /// Returns the integer's specification
    pub fn spec(&self) -> &Arc<IntegerSpec> {
        self.adaptor.spec()
    }

    /// Sets the integer value as an unsigned 64 bit value.
    pub fn set_u64(&mut self, value: u64) -> Result<(), IntegerError> {
        self.adaptor.set_u64(value)
    }

    /// Returns the integer value as an unsigned 64 bit value.
    pub fn u64(&self) -> Result<u64, IntegerError> {
        self.adaptor.u64()
    }

    /// Sets the integer value as a signed 64 bit value.
    pub fn set_i64(&mut self, value: i64) -> Result<(), IntegerError> {
        self.adaptor.set_i64(value)
    }

    /// Returns the integer value as a signed 64 bit value.
    pub fn i64(&self) -> Result<i64, IntegerError> {
        self.adaptor.i64()
    }
}

impl Accessor for Integer {}

impl TryFrom<Variable> for u8 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u8)
    }
}

impl TryFrom<Variable> for i8 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i8)
    }
}
impl TryFrom<Variable> for u16 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u16)
    }
}
impl TryFrom<Variable> for i16 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i16)
    }
}
impl TryFrom<Variable> for u32 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result as u32)
    }
}
impl TryFrom<Variable> for i32 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result as i32)
    }
}
impl TryFrom<Variable> for u64 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.u64()?;
        Ok(result)
    }
}
impl TryFrom<Variable> for i64 {
    type Error = IntegerError;

    fn try_from(value: Variable) -> Result<Self, Self::Error> {
        let int = value.integer();
        let result = int.i64()?;
        Ok(result)
    }
}
// Convert all integer types from a Variable
impl TryFrom<u8> for Variable {
    type Error = IntegerError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B8)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i8> for Variable {
    type Error = IntegerError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B8)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u16> for Variable {
    type Error = IntegerError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B16)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i16> for Variable {
    type Error = IntegerError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B16)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u32> for Variable {
    type Error = IntegerError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B32)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_u64(value as u64)?;
        Ok(var)
    }
}
impl TryFrom<i32> for Variable {
    type Error = IntegerError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B32)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_i64(value as i64)?;
        Ok(var)
    }
}
impl TryFrom<u64> for Variable {
    type Error = IntegerError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B64)
            .set_encoding(IntegerEncoding::Unsigned)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_u64(value)?;
        Ok(var)
    }
}
impl TryFrom<i64> for Variable {
    type Error = IntegerError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let spec = IntegerSpecBuilder::new()
            .set_storage(IntegerStorage::B64)
            .set_encoding(IntegerEncoding::Signed)
            .build();
        let mut var = default_data_provider().variable_for(&spec);
        let int = var.integer_mut();
        int.set_i64(value)?;
        Ok(var)
    }
}

/// An integer error.
#[derive(Debug, PartialEq)]
pub enum IntegerError {
    /// A provider error
    ProviderError(ProviderError),
    /// An overflow error
    Overflow(String),
}

impl Error for IntegerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IntegerError::ProviderError(e) => Some(e),
            IntegerError::Overflow(_) => None,
        }
    }
}

impl Display for IntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerError::ProviderError(err) => write!(f, "{:?}", err),
            IntegerError::Overflow(msg) => write!(f, "Overflow Error: {}", msg),
        }
    }
}

impl From<String> for IntegerError {
    fn from(value: String) -> Self {
        IntegerError::ProviderError(ProviderError::General(value))
    }
}

impl From<ProviderError> for IntegerError {
    fn from(value: ProviderError) -> Self {
        IntegerError::ProviderError(value)
    }
}
