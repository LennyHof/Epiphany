use std::rc::Rc;

use crate::{
    accessors::collections::list::ListError,
    adaptors::sequence_adaptor::SequenceAdaptor,
    data_provider::{DataProvider, default_data_provider},
    primitive_specs::list_spec::ListSpec,
    spec_compatibility::SpecCompatibility,
    variable::Variable,
};

/// An adaptor for lists.
pub trait ListAdaptor {
    /// Returns the list's specification.
    fn spec(&self) -> &Rc<ListSpec>;

    /// Returns whether the list is of fixed size.
    fn is_fixed_size(&self) -> bool;

    /// Returns whether the list is of fixed capacity.
    fn fixed_capacity(&self) -> Option<usize>;

    /// Returns the length of the list.
    fn len(&self) -> usize;

    /// Returns the value at the specified index.
    fn get(&self, index: usize) -> Result<&Variable, ListError> {
        if index < self.len() {
            self.do_get(index)
        } else {
            Err(ListError::IndexOutOfBounds(index, self.len()))
        }
    }

    /// Returns the value at the specified index.
    fn do_get(&self, index: usize) -> Result<&Variable, ListError>;

    /// Returns a mutable reference to the value at the specified index.
    /// If the index is out of bounds, it will return an error.
    fn get_mut(&mut self, index: usize) -> Result<&mut Variable, ListError> {
        if index < self.len() {
            self.do_get_mut(index)
        } else {
            Err(ListError::IndexOutOfBounds(index, self.len()))
        }
    }
    /// Returns a mutable reference to the value at the specified index.
    fn do_get_mut(&mut self, index: usize) -> Result<&mut Variable, ListError>;

    /// Sets the value at the specified index.
    fn set(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        value
            .data_spec()
            .check_compatible_with(self.spec().element_spec().as_ref().unwrap().as_ref())?;
        if index < self.len() {
            self.do_set(index, value)
        } else {
            Err(ListError::IndexOutOfBounds(index, self.len()))
        }
    }

    /// Sets the value at the specified index.
    fn do_set(&mut self, index: usize, value: Variable) -> Result<(), ListError>;

    /// Appends a value to the end of the list.
    fn push(&mut self, value: Variable) -> Result<(), ListError> {
        value
            .data_spec()
            .check_compatible_with(self.spec().element_spec().as_ref().unwrap().as_ref())?;
        if self.is_fixed_size() {
            Err(ListError::FixedSizeViolation)
        } else {
            if let Some(c) = self.fixed_capacity() {
                if self.len() >= c {
                    return Err(ListError::FixedCapacityViolation(c));
                }
            }
            self.do_push(value)
        }
    }

    /// Appends a value to the end of the list.
    fn do_push(&mut self, value: Variable) -> Result<(), ListError>;

    /// Removes and returns the last value from the list.
    fn pop(&mut self) -> Result<Option<Variable>, ListError> {
        if self.len() == 0 {
            return Err(ListError::CannotPopOnEmpty);
        }
        if self.is_fixed_size() {
            Err(ListError::FixedSizeViolation)
        } else {
            self.do_pop()
        }
    }

    /// Removes and returns the last value from the list.
    fn do_pop(&mut self) -> Result<Option<Variable>, ListError>;

    /// Inserts a value at the specified index.
    /// If the index is greater than the current length, it will return an error.
    fn insert(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        value
            .data_spec()
            .check_compatible_with(self.spec().element_spec().as_ref().unwrap().as_ref())?;
        if self.is_fixed_size() {
            return Err(ListError::FixedSizeViolation);
        }
        if let Some(c) = self.fixed_capacity() {
            if self.len() >= c {
                return Err(ListError::FixedCapacityViolation(c));
            }
        }
        if index <= self.len() {
            self.do_insert(index, value)
        } else {
            Err(ListError::IndexOutOfBounds(index, self.len()))
        }
    }

    /// Inserts a value at the specified index.
    fn do_insert(&mut self, index: usize, value: Variable) -> Result<(), ListError>;

    /// Removes the value at the specified index.   
    /// Returns the removed value if it exists.
    fn remove(&mut self, index: usize) -> Result<(), ListError> {
        if self.is_fixed_size() {
            return Err(ListError::FixedSizeViolation);
        }
        if index < self.len() {
            self.do_remove(index)
        } else {
            Err(ListError::IndexOutOfBounds(index, self.len()))
        }
    }

    /// Removes the value at the specified index.
    fn do_remove(&mut self, index: usize) -> Result<(), ListError>;

    /// Clears the list.
    fn clear(&mut self) -> Result<(), ListError> {
        if self.is_fixed_size() {
            Err(ListError::FixedSizeViolation)
        } else {
            self.do_clear()
        }
    }

    /// Clears the list.
    fn do_clear(&mut self) -> Result<(), ListError>;

    /// Returns the lists' capacity.
    /// Note: This is not the same as the length, as it may be larger than the current number of elements.
    fn capacity(&self) -> usize;

    /// Returns the lists' elements as a sequence.
    fn elements(&self) -> Box<dyn SequenceAdaptor>;

    /// Returns a boxed clone of the list adaptor.
    fn box_clone(&self) -> Box<dyn ListAdaptor> {
        let mut new_adaptor = default_data_provider().list_adaptor(self.spec());
        let mut i = 0;
        while i < self.len() {
            let elem = self.get(i).unwrap().clone();
            new_adaptor.push(elem).unwrap();
            i += 1;
        }
        new_adaptor
    }
}
