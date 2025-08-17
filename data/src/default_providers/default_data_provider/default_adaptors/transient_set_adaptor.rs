use crate::{
    accessors::collections::set::{SetError, SetIter},
    adaptors::collection_adaptors::set_adaptor::SetAdaptor,
    primitive_specs::set_spec::{SetElementOrdering, SetSpec},
    variable::Variable,
};
use std::{collections::BTreeSet, collections::HashSet, rc::Rc};

enum SetCollection {
    HashSet(HashSet<Variable>),
    BTreeSet(BTreeSet<Variable>),
}

pub struct TransientSetAdaptor {
    spec: Rc<SetSpec>,
    items: SetCollection,
}

impl TransientSetAdaptor {
    pub fn new(spec: Rc<SetSpec>) -> Self {
        Self {
            spec: spec.clone(),
            items: match spec.element_ordering().as_ref() {
                Some(SetElementOrdering::Unordered) => SetCollection::HashSet(HashSet::new()),
                Some(SetElementOrdering::Ordered) => SetCollection::BTreeSet(BTreeSet::new()),
                None => SetCollection::HashSet(HashSet::new()), // Default to HashSet if no element_ordering type is specified
            },
        }
    }
}

impl SetAdaptor for TransientSetAdaptor {
    fn spec(&self) -> &Rc<SetSpec> {
        &self.spec
    }

    fn len(&self) -> usize {
        match &self.items {
            SetCollection::HashSet(set) => set.len(),
            SetCollection::BTreeSet(set) => set.len(),
        }
    }

    fn contains(&self, value: &Variable) -> Result<bool, SetError> {
        match &self.items {
            SetCollection::HashSet(set) => Ok(set.contains(value)),
            SetCollection::BTreeSet(set) => Ok(set.contains(value)),
        }
    }

    fn do_insert(&mut self, value: Variable) -> Result<bool, SetError> {
        match &mut self.items {
            SetCollection::HashSet(set) => Ok(set.insert(value)),
            SetCollection::BTreeSet(set) => Ok(set.insert(value)),
        }
    }

    fn do_remove(&mut self, value: &Variable) -> Result<bool, SetError> {
        match &mut self.items {
            SetCollection::HashSet(set) => Ok(set.remove(value)),
            SetCollection::BTreeSet(set) => Ok(set.remove(value)),
        }
    }

    fn clear(&mut self) -> Result<(), SetError> {
        match &mut self.items {
            SetCollection::HashSet(set) => set.clear(),
            SetCollection::BTreeSet(set) => set.clear(),
        };
        Ok(())
    }

    fn values<'a>(&'a self) -> Box<dyn SetIter<'a> + 'a> {
        Box::new(ValueIter::new(&self.items))
    }
}

enum SetValueIter<'a> {
    HashSet(std::collections::hash_set::Iter<'a, Variable>),
    BTreeSet(std::collections::btree_set::Iter<'a, Variable>),
}

struct ValueIter<'a> {
    iter: SetValueIter<'a>,
}

impl<'a> ValueIter<'a> {
    fn new(items: &'a SetCollection) -> Self {
        Self {
            iter: match items {
                SetCollection::HashSet(set) => SetValueIter::HashSet(set.iter()),
                SetCollection::BTreeSet(set) => SetValueIter::BTreeSet(set.iter()),
            },
        }
    }
}

impl<'a> std::iter::Iterator for ValueIter<'a> {
    type Item = Result<&'a Variable, SetError>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter {
            SetValueIter::HashSet(iter) => iter.next().map(Ok),
            SetValueIter::BTreeSet(iter) => iter.next().map(Ok),
        }
    }
}

impl<'a> SetIter<'a> for ValueIter<'a> {}
