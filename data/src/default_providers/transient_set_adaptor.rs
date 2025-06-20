use crate::{
    accessors::{collections::set::SetError, sequence::SequenceIter},
    adaptors::{collection_adaptors::set_adaptor::SetAdaptor, sequence_adaptor::SequenceAdaptor},
    primitive_specs::{
        sequence_spec::SequenceSpec,
        set_spec::{SetElementOrdering, SetSpec},
    },
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

    fn values(&self) -> Box<dyn SequenceAdaptor> {
        let spec = Rc::new(SequenceSpec::new(self.spec.value_spec()));
        match &self.items {
            SetCollection::HashSet(set) => Box::new(ElementSequence::new(
                spec,
                SetCollection::HashSet(set.clone()),
            )),
            SetCollection::BTreeSet(set) => Box::new(ElementSequence::new(
                spec,
                SetCollection::BTreeSet(set.clone()),
            )),
        }
    }
}

struct ElementSequence {
    spec: Rc<SequenceSpec>,
    items: SetCollection,
}

impl ElementSequence {
    fn new(spec: Rc<SequenceSpec>, items: SetCollection) -> Self {
        Self {
            spec: spec.clone(),
            items: items,
        }
    }
}

impl SequenceAdaptor for ElementSequence {
    fn spec(&self) -> &Rc<SequenceSpec> {
        &self.spec
    }

    fn iter(&self) -> Box<dyn SequenceIter> {
        // Create an iterator based on the type of set collection
        match &self.items {
            SetCollection::HashSet(set) => Box::new(ElementSequenceIter {
                iterator: Box::new(set.clone().into_iter()),
            }),
            SetCollection::BTreeSet(set) => Box::new(ElementSequenceIter {
                iterator: Box::new(set.clone().into_iter()),
            }),
        }
    }
}

struct ElementSequenceIter {
    iterator: Box<dyn Iterator<Item = Variable>>,
}

impl std::iter::Iterator for ElementSequenceIter {
    type Item = Result<Variable, crate::accessors::sequence::SequenceError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(Ok)
    }
}

impl SequenceIter for ElementSequenceIter {}
