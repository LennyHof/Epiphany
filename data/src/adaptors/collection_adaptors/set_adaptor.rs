use std::rc::Rc;

use crate::{
    accessors::collections::set::SetError, adaptors::sequence_adaptor::SequenceAdaptor,
    primitive_specs::set_spec::SetSpec, spec_compatibility::SpecCompatibility, variable::Variable,
};

/// An adaptor for sets.
pub trait SetAdaptor {
    /// Returns the set's specification.
    fn spec(&self) -> &Rc<SetSpec>;

    /// Returns the length of the set.
    fn len(&self) -> usize;

    /// Checks if the set contains a value.
    /// Returns `true` if the value is in the set, `false` otherwise.
    fn contains(&self, value: &Variable) -> Result<bool, SetError>;

    /// Adds a value to the set. Returns `true` if the value was added, `false` if it was already present.
    ///
    /// This method checks if the value's data specification is compatible with the set's value specification.
    /// If the value's data specification is not compatible, it returns a `SetError::SpecError`.
    fn insert(&mut self, value: Variable) -> Result<bool, SetError> {
        // Check if the value's data specification is compatible with the set's value specification.
        value
            .data_spec()
            .check_compatible_with(self.spec().value_spec().as_ref().unwrap().as_ref())?;

        self.do_insert(value)
    }

    /// Adds a value to the set. Returns `true` if the value was added, `false` if it was already present.
    fn do_insert(&mut self, value: Variable) -> Result<bool, SetError>;

    /// Removes a value from the set.
    /// This method checks if the value's data specification is compatible with the set's value specification.
    /// If the value's data specification is not compatible, it returns a `SetError::SpecError`.
    fn remove(&mut self, value: &Variable) -> Result<bool, SetError> {
        // Check if the value's data specification is compatible with the set's value specification.
        value
            .data_spec()
            .check_compatible_with(self.spec().value_spec().as_ref().unwrap().as_ref())?;

        self.do_remove(value)
    }

    /// Removes a value from the set.
    fn do_remove(&mut self, value: &Variable) -> Result<bool, SetError>;

    /// Clears the set.
    fn clear(&mut self) -> Result<(), SetError>;

    /// Returns the lists' values as a sequence.
    fn values(&self) -> Box<dyn SequenceAdaptor>;
}
