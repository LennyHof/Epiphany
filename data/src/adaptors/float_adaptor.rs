use std::sync::Arc;

use crate::{
    accessors::float::FloatError,
    primitive_specs::float_spec::{FloatSpec, FloatStorage},
};

/// An adaptor for floats.
pub trait FloatAdaptor {
    /// Returns the float's specification.
    fn spec(&self) -> &Arc<FloatSpec>;

    /// Sets the float value as a 64 bit value.
    fn set_f64(&mut self, value: f64) -> Result<(), FloatError> {
        // Check if the value is within the range of the float spec
        self.check_for_overflow(value)?;
        self.do_set_f64(value)
    }

    /// Performs the sets of the float value as a 64 bit value.
    fn do_set_f64(&mut self, value: f64) -> Result<(), FloatError>;

    /// Returns the float value as a 64 bit value.
    fn f64(&self) -> Result<f64, FloatError>;

    /// Checks if the provided f64 value is within the valid range for the float storage type.
    fn check_for_overflow(&self, value: f64) -> Result<(), FloatError> {
        assert!(self.spec().storage().is_some());
        match self.spec().storage().unwrap() {
            FloatStorage::B32 => {
                let min_value = f32::MIN as f64;
                let max_value = f32::MAX as f64;
                if value < min_value || value > max_value {
                    return Err(FloatError::Overflow(
                        format!(
                            "value {} is out of {} storage range of {} to {}.",
                            value,
                            FloatStorage::B32,
                            min_value,
                            max_value
                        )
                        .to_string(),
                    ));
                }
            }
            FloatStorage::B64 => {}
        }
        Ok(())
    }
}
