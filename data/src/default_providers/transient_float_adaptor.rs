use std::rc::Rc;

use crate::{
    accessors::float::FloatError, adaptor::Adaptor, adaptors::float_adaptor::FloatAdaptor,
    primitive_specs::float_spec::FloatSpec,
};

enum FloatValue {
    B32(f32),
    B64(f64),
}

pub struct TransientFloatAdaptor {
    spec: Rc<FloatSpec>,
    value: FloatValue,
}

impl TransientFloatAdaptor {
    pub fn new(spec: Rc<FloatSpec>) -> TransientFloatAdaptor {
        TransientFloatAdaptor {
            spec,
            value: FloatValue::B32(0.0),
        }
    }
}

impl Adaptor for TransientFloatAdaptor {}

impl FloatAdaptor for TransientFloatAdaptor {
    fn spec(&self) -> &Rc<FloatSpec> {
        &self.spec
    }

    fn do_set_f64(&mut self, value: f64) -> Result<(), FloatError> {
        match self.spec.storage().unwrap() {
            crate::primitive_specs::float_spec::FloatStorage::B32 => {
                self.value = FloatValue::B32(value as f32);
            }
            crate::primitive_specs::float_spec::FloatStorage::B64 => {
                self.value = FloatValue::B64(value);
            }
        }
        Ok(())
    }

    fn f64(&self) -> Result<f64, FloatError> {
        match &self.value {
            FloatValue::B32(value) => Ok(*value as f64),
            FloatValue::B64(value) => Ok(*value),
        }
    }
}
