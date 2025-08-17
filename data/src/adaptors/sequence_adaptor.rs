use std::rc::Rc;

use crate::{
    accessors::sequence::{Sequence, SequenceError, SequenceIter},
    primitive_specs::sequence_spec::SequenceSpec,
    set_equal_to::SetEqualToError,
};

/// An adaptor for sequences.
pub trait SequenceAdaptor {
    /// Returns the sequence's specification.
    fn spec(&self) -> &Rc<SequenceSpec>;

    /// Returns a sequence iterator.
    fn iter<'a>(&'a self) -> Box<dyn SequenceIter<'a> + 'a>;

    /// Sets the sequence equal to another sequence.
    ///
    /// Returns and error if the other sequence's specification is incompatible,
    /// or if the sequence is read-only.
    fn set_equal_to(&mut self, other: &Sequence) -> Result<(), SetEqualToError> {
        if self.spec() != other.spec() {
            return Err(SetEqualToError::SequenceError(SequenceError::SpecError(
                crate::spec_compatibility::SpecError::IncompatibleSpec(
                    self.spec().to_string(),
                    other.spec().to_string(),
                ),
            )));
        }
        self.do_set_equal_to(other)
    }

    /// Internal method to set the sequence equal to another sequence.
    /// This method should be implemented by the adaptor if it supports setting equal to.
    fn do_set_equal_to(&mut self, _other: &Sequence) -> Result<(), SetEqualToError> {
        Err(SetEqualToError::SequenceError(SequenceError::ReadOnlyError))
    }
}
