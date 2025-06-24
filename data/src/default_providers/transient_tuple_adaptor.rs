use std::rc::Rc;

use crate::{
    adaptors::tuple_adaptor::TupleAdaptor, primitive_specs::tuple_spec::TupleSpec,
    variable::Variable,
};

pub struct TransientTupleAdaptor {
    spec: Rc<TupleSpec>,
    values: Vec<Variable>,
}

impl TransientTupleAdaptor {
    pub fn new(spec: Rc<TupleSpec>) -> Self {
        let mut values = Vec::with_capacity(spec.len());
        for i in 0..spec.len() {
            values.push(Variable::new(&spec.value_specs().as_ref().unwrap()[i]));
        }
        Self { spec, values }
    }
}

impl TupleAdaptor for TransientTupleAdaptor {
    fn spec(&self) -> &Rc<TupleSpec> {
        &self.spec
    }

    fn do_get(&self, index: usize) -> Result<&Variable, crate::accessors::tuple::TupleError> {
        Ok(&self.values[index])
    }

    fn do_get_mut(
        &mut self,
        index: usize,
    ) -> Result<&mut Variable, crate::accessors::tuple::TupleError> {
        Ok(&mut self.values[index])
    }

    fn do_set(
        &mut self,
        index: usize,
        value: Variable,
    ) -> Result<(), crate::accessors::tuple::TupleError> {
        self.values[index] = value;
        Ok(())
    }
}
