use std::sync::Arc;

use crate::{
    accessors::integer::IntegerError,
    adaptor::Adaptor,
    primitive_specs::integer_spec::{IntegerSpec, IntegerStorage},
};

/// Adaptor for integer values.
pub trait IntegerAdaptor: Adaptor {
    /// Returns the integer's specification.
    fn spec(&self) -> &Arc<IntegerSpec>;

    /// Sets the integer value as an unsigned 64 bit value.
    fn set_u64(&mut self, value: u64) -> Result<(), IntegerError> {
        // Check if the value is within the range of the integer spec
        self.check_for_unsigned_overflow(value)?;
        self.do_set_u64(value)
    }
    /// Performs the sets of the integer value as an unsigned 64 bit value.
    fn do_set_u64(&mut self, value: u64) -> Result<(), IntegerError>;
    /// Returns the integer value as an unsigned 64 bit value.
    fn u64(&self) -> Result<u64, IntegerError>;

    /// Sets the integer value as a signed 64 bit value.
    fn set_i64(&mut self, value: i64) -> Result<(), IntegerError> {
        // Check if the value is within the range of the integer spec
        self.check_for_signed_overflow(value)?;
        self.do_set_i64(value)
    }
    /// Performs the sets of the integer value as a signed 64 bit value.
    fn do_set_i64(&mut self, value: i64) -> Result<(), IntegerError>;
    /// Returns the integer value as a signed 64 bit value.
    fn i64(&self) -> Result<i64, IntegerError>;

    /// Checks if the given unsigned integer value is within the range defined by the integer specification.
    ///
    /// # Arguments
    /// * `value` - The unsigned integer value to check.
    ///
    /// # Returns
    /// * `Ok(())` if the value is within the range.
    /// * `Err(IntegerError::Overflow)` if the value is out of range.
    fn check_for_unsigned_overflow(&self, value: u64) -> Result<(), IntegerError> {
        assert!(!self.spec().is_signed());
        match self.spec().storage().unwrap() {
            IntegerStorage::B8 => {
                self.check_unsigned_overflow(value, IntegerStorage::B8, u8::MAX as u64)?
            }
            IntegerStorage::B16 => {
                self.check_unsigned_overflow(value, IntegerStorage::B16, u16::MAX as u64)?
            }
            IntegerStorage::B32 => {
                self.check_unsigned_overflow(value, IntegerStorage::B32, u32::MAX as u64)?
            }
            IntegerStorage::B64 => return Ok(()),
        }
        Ok(())
    }

    /// Check for signed overflow.
    /// Panics if the integer spec is not signed.
    fn check_for_signed_overflow(&self, value: i64) -> Result<(), IntegerError> {
        assert!(self.spec().is_signed());
        match self.spec().storage().unwrap() {
            IntegerStorage::B8 => self.check_signed_overflow(
                value,
                IntegerStorage::B8,
                i8::MIN as i64,
                i8::MAX as i64,
            )?,
            IntegerStorage::B16 => self.check_signed_overflow(
                value,
                IntegerStorage::B16,
                i16::MIN as i64,
                i16::MAX as i64,
            )?,
            IntegerStorage::B32 => self.check_signed_overflow(
                value,
                IntegerStorage::B32,
                i32::MIN as i64,
                i32::MAX as i64,
            )?,
            IntegerStorage::B64 => return Ok(()),
        }
        Ok(())
    }

    /// Checks if the given signed integer value is within the specified storage range.
    ///
    /// # Arguments
    /// * `value` - The signed integer value to check.
    /// * `storage` - The integer storage type (e.g., B8, B16, B32, B64).
    /// * `min_value` - The minimum allowed value for the storage type.
    /// * `max_value` - The maximum allowed value for the storage type.
    ///
    /// # Returns
    /// * `Ok(())` if the value is within the range.
    /// * `Err(IntegerError::Overflow)` if the value is out of range.
    fn check_signed_overflow(
        &self,
        value: i64,
        storage: IntegerStorage,
        min_value: i64,
        max_value: i64,
    ) -> Result<(), IntegerError> {
        if value < min_value || value > max_value {
            return Err(IntegerError::Overflow(
                format!(
                    "Value {} is out of {} storage range of {} to {}.",
                    value, storage, min_value, max_value
                )
                .to_string(),
            ));
        }
        Ok(())
    }

    /// Checks if the given unsigned integer value is within the specified storage range.
    ///
    /// # Arguments
    /// * `value` - The unsigned integer value to check.
    /// * `storage` - The integer storage type (e.g., B8, B16, B32, B64).
    /// * `max_value` - The maximum allowed value for the storage type.
    ///
    /// # Returns
    /// * `Ok(())` if the value is within the range.
    /// * `Err(IntegerError::Overflow)` if the value is out of range.
    fn check_unsigned_overflow(
        &self,
        value: u64,
        storage: IntegerStorage,
        max_value: u64,
    ) -> Result<(), IntegerError> {
        if value > max_value {
            return Err(IntegerError::Overflow(
                format!(
                    "Value {} is out of {} storage range of 0 to {}.",
                    value, storage, max_value
                )
                .to_string(),
            ));
        }
        Ok(())
    }
}
