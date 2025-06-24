use std::rc::Rc;

use crate::{
    accessors::collections::list::{ListError, ListIter, ListIterMut},
    adaptors::collection_adaptors::list_adaptor::ListAdaptor,
    data_provider::{DataProvider, default_data_provider},
    primitive_specs::list_spec::{ListSpec, ListStorage},
    variable::Variable,
};

pub struct TransientListAdaptor {
    /// The specification of the list.  
    /// This is used to define the type of items in the list.
    spec: Rc<ListSpec>,
    /// The list of items in the adaptor.
    items: Vec<Variable>,
    /// The maximum size of the list, if applicable.
    /// This is used to enforce size limits on the list.
    fixed_capacity: Option<usize>,
    /// Indicates whether the list is of fixed size.
    /// This is used to determine if the list can grow or shrink.
    is_fixed_size: bool,
}

impl TransientListAdaptor {
    /// Creates a new TransientListAdaptor with the given specification.
    pub fn new(spec: Rc<ListSpec>) -> Self {
        let mut items = Vec::new();
        let mut is_fixed_size: bool = false;
        let mut fixed_capacity: Option<usize> = None;
        if spec.storage().is_some() {
            match spec.storage().unwrap() {
                ListStorage::FixedSize(size) => {
                    if size == 0 {
                        panic!("Cannot create a TransientListAdaptor with a fixed size of 0.");
                    }
                    items.resize_with(size as usize, || {
                        default_data_provider().variable_for(spec.value_spec().as_ref().unwrap())
                    });
                    is_fixed_size = true;
                }
                ListStorage::FixedCapacity(capacity) => {
                    if capacity == 0 {
                        panic!("Cannot create a TransientListAdaptor with a fixed capacity of 0.");
                    }
                    items.reserve_exact(capacity as usize);
                    fixed_capacity = Some(capacity as usize);
                }
                ListStorage::InitialCapacity(capacity) => {
                    if capacity == 0 {
                        panic!(
                            "Cannot create a TransientListAdaptor with an initial capacity of 0."
                        );
                    }
                    items.reserve(capacity as usize);
                }
                ListStorage::VariableSize => {
                    // Variable size lists can be created without any restrictions.
                }
            }
        }
        TransientListAdaptor {
            items,
            spec,
            fixed_capacity,
            is_fixed_size,
        }
    }
}

impl ListAdaptor for TransientListAdaptor {
    fn spec(&self) -> &Rc<ListSpec> {
        &self.spec
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn do_get(&self, index: usize) -> Result<&Variable, ListError> {
        Ok(&self.items[index])
    }

    fn do_get_mut(&mut self, index: usize) -> Result<&mut Variable, ListError> {
        Ok(&mut self.items[index])
    }

    fn do_set(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        self.items[index] = value;
        Ok(())
    }

    fn do_push(&mut self, value: Variable) -> Result<(), ListError> {
        self.items.push(value);
        Ok(())
    }

    fn do_pop(&mut self) -> Result<Option<Variable>, ListError> {
        Ok(self.items.pop())
    }

    fn do_insert(&mut self, index: usize, value: Variable) -> Result<(), ListError> {
        self.items.insert(index, value);
        Ok(())
    }

    fn do_remove(&mut self, index: usize) -> Result<(), ListError> {
        self.items.remove(index);
        Ok(())
    }

    fn do_clear(&mut self) -> Result<(), ListError> {
        self.items.clear();
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.items.capacity()
    }

    fn is_fixed_size(&self) -> bool {
        self.is_fixed_size
    }

    fn fixed_capacity(&self) -> Option<usize> {
        self.fixed_capacity
    }

    fn iter<'a>(&'a self) -> Box<dyn ListIter<'a> + 'a> {
        Box::new(ValueIter {
            iter: self.items.iter(),
        })
    }

    fn iter_mut<'a>(&'a mut self) -> Box<dyn ListIterMut<'a> + 'a> {
        Box::new(ValueIterMut {
            iter: self.items.iter_mut(),
        })
    }
}

struct ValueIter<'a> {
    iter: std::slice::Iter<'a, Variable>,
}

impl<'a> std::iter::Iterator for ValueIter<'a> {
    type Item = Result<&'a Variable, ListError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Ok)
    }
}

impl<'a> ListIter<'a> for ValueIter<'a> {}

struct ValueIterMut<'a> {
    iter: std::slice::IterMut<'a, Variable>,
}

impl<'a> std::iter::Iterator for ValueIterMut<'a> {
    type Item = Result<&'a mut Variable, ListError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Ok)
    }
}

impl<'a> ListIterMut<'a> for ValueIterMut<'a> {}
