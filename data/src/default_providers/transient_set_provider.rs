use crate::{
    accessors::collections::set::SetError, adaptors::collection_adaptors::set_adaptor::SetAdaptor,
    primitive_specs::set_spec::SetSpec, variable::Variable,
};
use std::{collections::HashSet, rc::Rc};

pub struct TransientSetProvider {
    spec: Rc<SetSpec>,
    items: HashSet<Variable>,
}

impl TransientSetProvider {
    pub fn new(spec: Rc<SetSpec>) -> Self {
        Self {
            spec,
            items: HashSet::new(),
        }
    }
}

impl SetAdaptor for TransientSetProvider {
    fn spec(&self) -> &Rc<SetSpec> {
        &self.spec
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn contains(&self, value: &Variable) -> Result<bool, SetError> {
        todo!()
        // value
        //     .data_spec()
        //     .check_compatible_with(self.spec.element_spec().as_ref().unwrap())?;
        // Ok(self.items.contains(value))
    }

    fn do_add(&mut self, value: Variable) -> Result<(), SetError> {
        todo!()
        // if !self.items.insert(value) {
        //     return Err(SetError::AlreadyExists);
        // }
        // Ok(())
    }

    fn do_remove(&mut self, value: &Variable) -> Result<bool, SetError> {
        todo!()
        //Ok(self.items.remove(value))
    }

    fn clear(&mut self) -> Result<(), SetError> {
        self.items.clear();
        Ok(())
    }
}
