use std::rc::Rc;

use crate::{
    accessors::integer::IntegerError, adaptor::Adaptor, adaptors::integer_adaptor::IntegerAdaptor,
    primitive_specs::integer_spec::IntegerSpec,
};

enum IntegerValue {
    Unsigned(u64),
    Signed(i64),
}

pub struct TransientIntegerAdaptor {
    spec: Rc<IntegerSpec>,
    value: IntegerValue,
}

impl TransientIntegerAdaptor {
    pub fn new(spec: Rc<IntegerSpec>) -> Self {
        TransientIntegerAdaptor {
            spec: (spec.clone()),
            value: (if spec.is_signed() {
                IntegerValue::Signed(0)
            } else {
                IntegerValue::Unsigned(0)
            }),
        }
    }
}

impl Adaptor for TransientIntegerAdaptor {}

impl IntegerAdaptor for TransientIntegerAdaptor {
    fn spec(&self) -> &Rc<IntegerSpec> {
        &self.spec
    }

    fn do_set_u64(&mut self, value: u64) -> Result<(), IntegerError> {
        assert!(!self.spec.is_signed());
        self.value = IntegerValue::Unsigned(value);
        Ok(())
    }

    fn u64(&self) -> Result<u64, IntegerError> {
        match self.value {
            IntegerValue::Unsigned(value) => Ok(value),
            IntegerValue::Signed(_) => panic!(),
        }
    }

    fn do_set_i64(&mut self, value: i64) -> Result<(), IntegerError> {
        assert!(self.spec.is_signed());
        self.value = IntegerValue::Signed(value);
        Ok(())
    }

    fn i64(&self) -> Result<i64, IntegerError> {
        match self.value {
            IntegerValue::Signed(value) => Ok(value),
            IntegerValue::Unsigned(_) => panic!(),
        }
    }
}
