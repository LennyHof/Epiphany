use crate::{
    accessors::collections::list::ListError,
    data_provider::{DataProvider, default_data_provider},
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, list_spec_builder::ListSpecBuilder,
    },
    primitive_specs::{
        integer_spec::{IntegerEncoding, IntegerStorage},
        list_spec::ListStorage,
    },
    spec_compatibility::SpecError,
    variable::Variable,
};

#[test]
fn variable_list() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    // test element values
    let elem0_in = 55i64;
    let elem1_in = 132i64;
    // populate using push
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    assert!(list.is_empty());
    list.push(Variable::try_from(elem0_in).unwrap()).unwrap();
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    list.push(Variable::try_from(elem1_in).unwrap()).unwrap();
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), elem0_in);
    assert_eq!(list.get(1).unwrap().integer().i64().unwrap(), elem1_in);
    // pop
    let pop1 = list.pop().unwrap().unwrap();
    assert_eq!(pop1.integer().i64().unwrap(), elem1_in);
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    // pop again
    let pop0 = list.pop().unwrap().unwrap();
    assert_eq!(pop0.integer().i64().unwrap(), elem0_in);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    // populate again using push and insert
    list.push(Variable::try_from(elem1_in).unwrap()).unwrap();
    list.insert(0, Variable::try_from(elem0_in).unwrap())
        .unwrap();
    assert_eq!(list.len(), 2);
    assert!(!list.is_empty());
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), elem0_in);
    assert_eq!(list.get(1).unwrap().integer().i64().unwrap(), elem1_in);
    // remove
    list.remove(0).unwrap();
    assert_eq!(list.len(), 1);
    assert!(!list.is_empty());
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), elem1_in);
    // clear
    list.clear().unwrap();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn variable_size_list_violations() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    // populate using push
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 2);
    // get out of bounds
    assert_eq!(
        list.get(44).unwrap_err(),
        ListError::IndexOutOfBounds(44, 2)
    );
    // get_mut out of bounds
    assert_eq!(
        list.get_mut(44).unwrap_err(),
        ListError::IndexOutOfBounds(44, 2)
    );
    // insert out of bounds
    assert_eq!(
        list.insert(44, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::IndexOutOfBounds(44, 2)
    );
    // set out of bounds
    assert_eq!(
        list.set(44, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::IndexOutOfBounds(44, 2)
    );
    // remove out of bounds
    assert_eq!(
        list.remove(44).unwrap_err(),
        ListError::IndexOutOfBounds(44, 2)
    );
    // pop on an empty list
    let mut empty_list = default_data_provider().variable_for(&spec);
    let empty_list = empty_list.list_mut();
    assert_eq!(empty_list.pop().unwrap_err(), ListError::CannotPopOnEmpty);
    // set an element with a different data spec
    assert_eq!(
        list.set(0, Variable::try_from(55u64).unwrap()).unwrap_err(),
        ListError::ElementSpecError(SpecError::IncompatibleSpec(
            "Integer { encoding: Unsigned, storage: B64 }".to_string(),
            "Integer { encoding: Signed, storage: B64 }".to_string()
        ))
    );
    // insert an element with a different data spec
    assert_eq!(
        list.insert(0, Variable::try_from(55u64).unwrap())
            .unwrap_err(),
        ListError::ElementSpecError(SpecError::IncompatibleSpec(
            "Integer { encoding: Unsigned, storage: B64 }".to_string(),
            "Integer { encoding: Signed, storage: B64 }".to_string()
        ))
    );
    // push an element with a different data spec
    assert_eq!(
        list.push(Variable::try_from(55u64).unwrap()).unwrap_err(),
        ListError::ElementSpecError(SpecError::IncompatibleSpec(
            "Integer { encoding: Unsigned, storage: B64 }".to_string(),
            "Integer { encoding: Signed, storage: B64 }".to_string()
        ))
    );
}

#[test]
fn fixed_size_list() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::FixedSize(2))
        .build();

    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();

    // verify that the list is the right length and it elements have the right initial value.
    assert_eq!(list.len(), 2);
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), 0);
    assert_eq!(list.get(1).unwrap().integer().i64().unwrap(), 0);

    // test element values
    let elem0_in = 55i64;
    let elem1_in = 132i64;

    // test setting existing element values
    list.get_mut(0)
        .unwrap()
        .integer_mut()
        .set_i64(elem0_in)
        .unwrap();
    list.get_mut(1)
        .unwrap()
        .integer_mut()
        .set_i64(elem1_in)
        .unwrap();
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), elem0_in);
    assert_eq!(list.get(1).unwrap().integer().i64().unwrap(), elem1_in);
}

#[test]
fn fixed_size_list_violations() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::FixedSize(2))
        .build();
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    // get out of bounds
    assert_eq!(list.get(3).unwrap_err(), ListError::IndexOutOfBounds(3, 2));
    // get_mut out of bounds
    assert_eq!(
        list.get_mut(3).unwrap_err(),
        ListError::IndexOutOfBounds(3, 2)
    );
    // set out of bounds
    assert_eq!(
        list.set(3, Variable::try_from(55i64).unwrap()).unwrap_err(),
        ListError::IndexOutOfBounds(3, 2)
    );
    // insert out of bounds
    assert_eq!(
        list.insert(3, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::FixedSizeViolation
    );
    // push
    assert_eq!(
        list.push(Variable::try_from(55i64).unwrap()).unwrap_err(),
        ListError::FixedSizeViolation
    );
    // insert
    assert_eq!(
        list.insert(0, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::FixedSizeViolation
    );
    // remove
    assert_eq!(list.remove(3).unwrap_err(), ListError::FixedSizeViolation);
    // pop
    assert_eq!(list.pop().unwrap_err(), ListError::FixedSizeViolation);
    // clear
    assert_eq!(list.clear().unwrap_err(), ListError::FixedSizeViolation);
}

#[test]
fn fixed_capacity_list() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::FixedCapacity(2))
        .build();
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    assert_eq!(list.capacity(), 2);
    // push
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list.capacity(), 2);
    // insert
    list.insert(0, Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list.capacity(), 2);
    // pop
    list.pop().unwrap().unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list.capacity(), 2);
    // remove
    list.remove(0).unwrap();
    assert_eq!(list.len(), 0);
    assert_eq!(list.capacity(), 2);
    // push to have clear work against a non-empty list
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list.capacity(), 2);
    // clear
    list.clear().unwrap();
    assert_eq!(list.capacity(), 2);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn fixed_capacity_list_violations() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::FixedCapacity(2))
        .build();
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    // fill the list to capacity
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    // get out of bounds
    assert_eq!(list.get(3).unwrap_err(), ListError::IndexOutOfBounds(3, 2));
    // get_mut out of bounds
    assert_eq!(
        list.get_mut(3).unwrap_err(),
        ListError::IndexOutOfBounds(3, 2)
    );
    // set out of bounds
    assert_eq!(
        list.set(3, Variable::try_from(55i64).unwrap()).unwrap_err(),
        ListError::IndexOutOfBounds(3, 2)
    );
    // insert out of bounds
    assert_eq!(
        list.insert(3, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::FixedCapacityViolation(2)
    );
    // push to exceed capacity
    assert_eq!(
        list.push(Variable::try_from(55i64).unwrap()).unwrap_err(),
        ListError::FixedCapacityViolation(2)
    );
    // insert at the beginning to exceed capacity
    assert_eq!(
        list.insert(0, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::FixedCapacityViolation(2)
    );
}

#[test]
fn initial_capacity_list() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::InitialCapacity(2))
        .build();
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    assert!(list.capacity() >= 2);
    // push
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 1);
    assert!(list.capacity() >= 2);
    // insert
    list.insert(0, Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.capacity() >= 2);
    // pop
    list.pop().unwrap().unwrap();
    assert_eq!(list.len(), 1);
    assert!(list.capacity() >= 2);
    // remove
    list.remove(0).unwrap();
    assert_eq!(list.len(), 0);
    assert!(list.capacity() >= 2);
    // push to have clear work against a non-empty list
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_eq!(list.len(), 1);
    assert!(list.capacity() >= 2);
    // clear
    list.clear().unwrap();
    assert!(list.capacity() >= 2);
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn initial_capacity_list_violations() {
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(ListStorage::InitialCapacity(2))
        .build();
    let mut var = default_data_provider().variable_for(&spec);
    let list = var.list_mut();
    // get out of bounds
    assert_eq!(list.get(3).unwrap_err(), ListError::IndexOutOfBounds(3, 0));
    // get_mut out of bounds
    assert_eq!(
        list.get_mut(3).unwrap_err(),
        ListError::IndexOutOfBounds(3, 0)
    );
    // set out of bounds
    assert_eq!(
        list.set(3, Variable::try_from(55i64).unwrap()).unwrap_err(),
        ListError::IndexOutOfBounds(3, 0)
    );
    // insert out of bounds
    assert_eq!(
        list.insert(3, Variable::try_from(55i64).unwrap())
            .unwrap_err(),
        ListError::IndexOutOfBounds(3, 0)
    );
}
