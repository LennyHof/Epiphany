use std::rc::Rc;

use crate::{
    accessors::sequence::{Sequence, SequenceError, SequenceIter},
    adaptors::sequence_adaptor::SequenceAdaptor,
    data_provider::{self, DataProvider},
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, sequence_spec_builder::SequenceSpecBuilder,
    },
    primitive_specs::sequence_spec::SequenceSpec,
    variable::Variable,
};

const ELEM0_VALUE: i64 = 55i64;
const ELEM1_VALUE: i64 = 132i64;

struct TestSequenceAdaptor {
    spec: Rc<SequenceSpec>,
    // For testing purposes, we can use a simple vector to hold the sequence items.
    items: Vec<Variable>,
}
impl TestSequenceAdaptor {
    pub fn new(spec: Rc<SequenceSpec>) -> Self {
        Self {
            spec: spec.clone(),
            items: vec![
                Variable::try_from(ELEM0_VALUE).unwrap(),
                Variable::try_from(ELEM1_VALUE).unwrap(),
            ],
        }
    }
}

impl SequenceAdaptor for TestSequenceAdaptor {
    fn spec(&self) -> &Rc<SequenceSpec> {
        &self.spec
    }

    fn iter<'a>(&'a self) -> Box<dyn SequenceIter<'a> + 'a> {
        Box::new(ValueIter {
            iter: self.items.iter(),
        })
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

struct TestDataProvider {}

impl DataProvider for TestDataProvider {
    fn sequence_adaptor(&self, _spec: &Rc<SequenceSpec>) -> Box<dyn SequenceAdaptor> {
        Box::new(TestSequenceAdaptor::new(Rc::clone(_spec)))
    }

    fn name(&self) -> String {
        "TestDataProvider".to_string()
    }
}

#[test]
fn test_sequence_adaptor() {
    let spec = Rc::new(SequenceSpec::new(&None));
    let adaptor = TestSequenceAdaptor::new(spec.clone());

    // Test iterating over the sequence
    let mut iter = adaptor.iter();
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM0_VALUE
    );
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM1_VALUE
    );
    assert!(iter.next().is_none());
}

#[test]
fn test_sequence() {
    let spec = SequenceSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(crate::primitive_specs::integer_spec::IntegerStorage::B64)
                .build(),
        )
        .build();

    let data_provider = TestDataProvider {};
    let var = data_provider.variable_for(&spec);

    // Test iterating over the sequence
    let mut iter = var.sequence().iter();
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM0_VALUE
    );
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM1_VALUE
    );
    assert!(iter.next().is_none());
}

#[test]
fn sequence_set_equal_to() {
    let spec = SequenceSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(crate::primitive_specs::integer_spec::IntegerStorage::B64)
                .build(),
        )
        .build();

    let data_provider = TestDataProvider {};
    let source = data_provider.variable_for(&spec);
    let target = source.try_clone().unwrap();

    // Test iterating over target's sequence
    let mut iter = target.sequence().iter();
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM0_VALUE
    );
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        ELEM1_VALUE
    );
    assert!(iter.next().is_none());
}
