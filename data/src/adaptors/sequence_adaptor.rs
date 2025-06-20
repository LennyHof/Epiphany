use std::rc::Rc;

use crate::{accessors::sequence::SequenceIter, primitive_specs::sequence_spec::SequenceSpec};

/// An adaptor for sequences.
pub trait SequenceAdaptor {
    /// Returns the sequence's specification.
    fn spec(&self) -> &Rc<SequenceSpec>;

    /// Returns a sequence iterator.
    /// This method should return an iterator that yields references to the values in the sequence.
    fn iter(&self) -> Box<dyn SequenceIter>;
}
