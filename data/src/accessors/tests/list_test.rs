use crate::{
    accessors::collections::list::ListError,
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, list_spec_builder::ListSpecBuilder,
    },
    primitive_specs::{
        integer_spec::{IntegerEncoding, IntegerStorage},
        list_spec::ListStorage,
    },
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecError,
    variable::Variable,
};

use std::hash::{DefaultHasher, Hash, Hasher};

#[test]
fn variable_list() {
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    // test element values
    let elem0_in = 55i64;
    let elem1_in = 132i64;
    // populate using push
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
    let mut var = Variable::new(&spec);
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
    let mut empty_list = Variable::new(&spec);
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

    let mut var = Variable::new(&spec);
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
    let mut var = Variable::new(&spec);
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
    let mut var = Variable::new(&spec);
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
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .set_storage(ListStorage::FixedCapacity(2))
            .build(),
    );
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
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .set_storage(ListStorage::InitialCapacity(2))
            .build(),
    );
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
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .set_storage(ListStorage::InitialCapacity(2))
            .build(),
    );
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

#[test]
fn list_element_sequence() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(element_spec.clone())
            .build(),
    );
    let list = var.list_mut();
    // test element values
    let elem0_in = 55i64;
    let elem1_in = 132i64;
    // populate using push
    list.push(Variable::try_from(elem0_in).unwrap()).unwrap();
    list.push(Variable::try_from(elem1_in).unwrap()).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list.get(0).unwrap().integer().i64().unwrap(), elem0_in);
    assert_eq!(list.get(1).unwrap().integer().i64().unwrap(), elem1_in);

    let elements = &list.elements();

    // check the sequence spec's element spec
    assert_eq!(
        elements.spec().element_spec().as_ref().unwrap().as_ref(),
        element_spec.as_ref()
    );
    // iterate over the list
    let mut iter = list.elements().iter();
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        elem0_in
    );
    assert_eq!(
        iter.next().unwrap().unwrap().integer().i64().unwrap(),
        elem1_in
    );
    assert_eq!(iter.next(), None);
    assert_eq!(list.len(), 2);
    let mut count = 0;
    for element in elements.iter() {
        if count == 0 {
            assert_eq!(element.unwrap().integer().i64().unwrap(), elem0_in);
        } else if count == 1 {
            assert_eq!(element.unwrap().integer().i64().unwrap(), elem1_in);
        }
        count += 1;
    }
    assert_eq!(count, 2);
}

#[test]
fn test_partial_eq_and_hash() {
    // Test that two lists with the same spec and elements are equal
    // and that they are not equal when one of them has a different element.
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);
    {
        let list1 = var1.list_mut();
        let list2 = var2.list_mut();

        assert_eq!(list1, list2);
        // check that the hash values of the lists are equal
        {
            let mut hasher1 = DefaultHasher::new();
            list1.hash(&mut hasher1);
            let mut hasher2 = DefaultHasher::new();
            list2.hash(&mut hasher2);
            assert_eq!(hasher1.finish(), hasher2.finish());
        }

        // make the lists unequal by adding an element to one of them
        list1.push(Variable::try_from(55i64).unwrap()).unwrap();

        assert_ne!(list1, list2);
        // check that the hash values of the lists are not equal anymore
        {
            let mut hasher1 = DefaultHasher::new();
            list1.hash(&mut hasher1);
            let mut hasher2 = DefaultHasher::new();
            list2.hash(&mut hasher2);
            assert_ne!(hasher1.finish(), hasher2.finish());
        }

        // make the lists equal again
        list2.push(Variable::try_from(55i64).unwrap()).unwrap();
    }
    // check variables are equal
    assert_eq!(var1, var2);
    // check that the hash values of the variables are equal
    {
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // add a different element to var2
    // this should make them unequal
    var2.list_mut()
        .push(Variable::try_from(132i64).unwrap())
        .unwrap();

    // check variables are no longer equal
    assert_ne!(var1, var2);
    // check that the hash values of the variables are not equal anymore
    {
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn set_equal_to() {
    // Test that two lists with the same spec and elements can be set equal to each other.
    let spec = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);

    let list1 = var1.list_mut();
    let list2 = var2.list_mut();

    assert_eq!(list1, list2);

    list1.push(Variable::try_from(55i64).unwrap()).unwrap();
    assert_ne!(list1, list2);

    list2.set_equal_to(&list1).unwrap();
    assert_eq!(list1, list2);
}

#[test]
fn set_equal_to_different_specs() {
    // Test that setting a list equal to another list with a different spec fails.
    let spec1 = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let spec2 = ListSpecBuilder::new()
        .set_element_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();

    let mut var1 = Variable::new(&spec1);
    let mut var2 = Variable::new(&spec2);

    let list1 = var1.list_mut();
    let list2 = var2.list_mut();

    assert_eq!(list1, list2);

    list1.push(Variable::try_from(55i64).unwrap()).unwrap();

    // Attempt to set list2 equal to list1 should fail due to incompatible specs
    assert_eq!(
        list2.set_equal_to(&list1).unwrap_err(),
        SetEqualToError::SpecError(SpecError::IncompatibleSpec(
            "List { element_spec: Integer { encoding: Unsigned, storage: B64 }, storage: None }"
                .to_string(),
            "List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: None }"
                .to_string()
        ))
    );
}

#[test]
fn list_display() {
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let list = var.list_mut();
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    list.push(Variable::try_from(132i64).unwrap()).unwrap();

    // Check the display output of the list and variable
    assert_eq!(format!("{}", list), "[55, 132]");
    assert_eq!(format!("{}", var), "[55, 132]");
}

#[test]
fn list_debug() {
    let mut var = Variable::new(
        &ListSpecBuilder::new()
            .set_element_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let list = var.list_mut();
    list.push(Variable::try_from(55i64).unwrap()).unwrap();
    list.push(Variable::try_from(132i64).unwrap()).unwrap();

    // Check the debug output of the list and variable
    assert_eq!(format!("{:?}", list), "List (length: 2, value: [55, 132])");
    assert_eq!(format!("{:?}", var), "spec: List { element_spec: Integer { encoding: Signed, storage: B64 }, storage: None }, value: [55, 132])");
}
