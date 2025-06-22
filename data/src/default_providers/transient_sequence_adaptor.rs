use std::rc::Rc;

use crate::{
    accessors::sequence::{Sequence, SequenceError, SequenceIter},
    adaptors::sequence_adaptor::SequenceAdaptor,
    primitive_specs::sequence_spec::SequenceSpec,
    set_equal_to::SetEqualToError,
    variable::Variable,
};

pub struct TransientSequenceAdaptor {
    spec: Rc<SequenceSpec>,
    items: Vec<Variable>,
}

impl TransientSequenceAdaptor {
    /// Creates a new TransientSequenceAdaptor with the given specification.
    pub fn new(spec: Rc<SequenceSpec>) -> Self {
        //let mut items = Vec::new();
        Self {
            spec,
            items: Vec::new(),
        }
    }
}

impl SequenceAdaptor for TransientSequenceAdaptor {
    fn spec(&self) -> &Rc<SequenceSpec> {
        &self.spec
    }

    fn iter<'a>(&'a self) -> Box<dyn SequenceIter<'a> + 'a> {
        Box::new(ValueIter {
            iter: self.items.iter(),
        })
    }

    fn do_set_equal_to(&mut self, other: &Sequence) -> Result<(), SetEqualToError> {
        self.items.clear();
        for value in other.iter() {
            match value {
                Ok(var) => self.items.push(var.clone()),
                Err(e) => return Err(SetEqualToError::SequenceError(e)),
            }
        }
        Ok(())
    }
}

struct ValueIter<'a> {
    iter: std::slice::Iter<'a, Variable>,
}

impl<'a> std::iter::Iterator for ValueIter<'a> {
    type Item = Result<&'a Variable, SequenceError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Ok)
    }
}

impl<'a> SequenceIter<'a> for ValueIter<'a> {}
