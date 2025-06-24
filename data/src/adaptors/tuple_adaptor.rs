use std::rc::Rc;

use crate::{
    accessors::tuple::TupleError, primitive_specs::tuple_spec::TupleSpec,
    spec_compatibility::SpecCompatibility, variable::Variable,
};

/// An adaptor for tuples.
pub trait TupleAdaptor {
    /// Returns the tuple's specification.
    fn spec(&self) -> &Rc<TupleSpec>;

    /// Returns the number of elements in the tuple
    fn len(&self) -> usize {
        self.spec().len()
    }

    /// Returns the value at the specified index.
    fn get(&self, index: usize) -> Result<&Variable, TupleError> {
        if index < self.len() {
            self.do_get(index)
        } else {
            Err(TupleError::IndexOutOfBounds(index, self.spec().len()))
        }
    }

    /// Returns the value at the specified index.
    fn do_get(&self, index: usize) -> Result<&Variable, TupleError>;

    /// Returns a mutable reference to the value at the specified index.
    /// If the index is out of bounds, it will return an error.
    fn get_mut(&mut self, index: usize) -> Result<&mut Variable, TupleError> {
        if index < self.len() {
            self.do_get_mut(index)
        } else {
            Err(TupleError::IndexOutOfBounds(index, self.spec().len()))
        }
    }
    /// Returns a mutable reference to the value at the specified index.
    fn do_get_mut(&mut self, index: usize) -> Result<&mut Variable, TupleError>;

    /// Sets the value at the specified index.
    fn set(&mut self, index: usize, value: Variable) -> Result<(), TupleError> {
        if index < self.len() {
            value.data_spec().check_compatible_with(
                self.spec().value_specs().as_ref().unwrap()[index].as_ref(),
            )?;
            self.do_set(index, value)
        } else {
            Err(TupleError::IndexOutOfBounds(index, self.len()))
        }
    }

    /// Sets the value at the specified index.
    fn do_set(&mut self, index: usize, value: Variable) -> Result<(), TupleError>;
}
