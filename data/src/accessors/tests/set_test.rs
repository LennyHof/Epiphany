use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, set_spec_builder::SetSpecBuilder,
    },
    primitive_specs::{
        integer_spec::{IntegerEncoding, IntegerStorage},
        set_spec::SetElementOrdering,
    },
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecError,
    variable::Variable,
};

#[test]
fn set_insert() {
    let mut var = Variable::new(
        &SetSpecBuilder::new()
            .set_value_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Unsigned)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let set = var.set_mut();
    assert!(set.is_empty());
    // test element values
    let elem0_in = 55u64;
    let elem1_in = 132u64;
    let elem2_in = 99u64;

    // add values
    set.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem1_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem2_in).unwrap()).unwrap();
    // check size
    assert_eq!(set.len(), 3);
    // check values
    assert!(
        set.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );
}

#[test]
fn set_insert_duplicate() {
    let mut var = Variable::new(
        &SetSpecBuilder::new()
            .set_value_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Unsigned)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let set = var.set_mut();
    assert!(set.is_empty());
    // test element values
    let elem_in = 55u64;

    // add element
    set.insert(Variable::try_from(elem_in).unwrap()).unwrap();
    // check size
    assert_eq!(set.len(), 1);
    // check element
    assert!(set.contains(&Variable::try_from(elem_in).unwrap()).unwrap());

    // try to add duplicate element
    let res = set.insert(Variable::try_from(elem_in).unwrap());
    assert!(res.is_ok());
    assert!(!res.unwrap());
    // check size
    assert_eq!(set.len(), 1);
}

#[test]
fn set_remove() {
    let mut var = Variable::new(
        &SetSpecBuilder::new()
            .set_value_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Unsigned)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let set = var.set_mut();
    assert!(set.is_empty());
    // test element values
    let elem0_in = 55u64;
    let elem1_in = 132u64;
    let elem2_in = 99u64;

    // add values
    set.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem1_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem2_in).unwrap()).unwrap();
    // check size
    assert_eq!(set.len(), 3);
    // check values
    assert!(
        set.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );

    // remove an element
    let res = set.remove(&Variable::try_from(elem1_in).unwrap()).unwrap();
    assert!(res);
    // check size
    assert_eq!(set.len(), 2);
    // check values
    assert!(
        set.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        !set.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );
    // try to remove an element that is not in the set
    let res = set.remove(&Variable::try_from(999u64).unwrap()).unwrap();
    assert!(!res);
    // check size
    assert_eq!(set.len(), 2);
    // check values
    assert!(
        set.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        !set.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );
}

#[test]
fn set_clear() {
    let mut var = Variable::new(
        &SetSpecBuilder::new()
            .set_value_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Unsigned)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .build(),
    );
    let set = var.set_mut();
    assert!(set.is_empty());
    // test element values
    let elem0_in = 55u64;
    let elem1_in = 132u64;
    let elem2_in = 99u64;

    // add values
    set.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem1_in).unwrap()).unwrap();
    set.insert(Variable::try_from(elem2_in).unwrap()).unwrap();
    // check size
    assert_eq!(set.len(), 3);
    // check values
    assert!(
        set.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );

    // clear the set
    set.clear().unwrap();
    // check size
    assert_eq!(set.len(), 0);
}

#[test]
fn set_iterators() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(SetElementOrdering::Ordered)
        .build();
    let mut var = Variable::new(&spec);
    {
        let set = var.set_mut();
        assert!(set.is_empty());

        // test element values
        let elem0_in = Variable::try_from(55u64).unwrap();
        let elem1_in = Variable::try_from(132u64).unwrap();
        let elem2_in = Variable::try_from(1u64).unwrap();

        // add values
        set.insert(elem0_in.clone()).unwrap();
        set.insert(elem1_in.clone()).unwrap();
        set.insert(elem2_in.clone()).unwrap();

        // check size
        assert_eq!(set.len(), 3);

        // iterate over values
        let mut values_iter = set.iter();
        assert_eq!(values_iter.next().unwrap().unwrap(), &elem2_in);
        assert_eq!(values_iter.next().unwrap().unwrap(), &elem0_in);
        assert_eq!(values_iter.next().unwrap().unwrap(), &elem1_in);
        assert_eq!(values_iter.next(), None);
    }

    // check values using into iter on a reference to the set
    let mut count = 0;
    for value in var.set() {
        assert!(value.is_ok());
        count += 1;
    }
    assert_eq!(count, 3);

    // check values using into iter on a mut reference to the set
    let mut count = 0;
    for value in var.set_mut() {
        assert!(value.is_ok());
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn ordered_set() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(SetElementOrdering::Ordered)
        .build();
    let mut var = Variable::new(&spec);
    let set = var.set_mut();

    // add values
    set.insert(Variable::try_from(55u64).unwrap()).unwrap();
    set.insert(Variable::try_from(132u64).unwrap()).unwrap();
    set.insert(Variable::try_from(99u64).unwrap()).unwrap();

    // check ordering
    assert_eq!(set.to_string(), "Set {55, 99, 132}");
}

#[test]
fn set_equal_to_compatible_specs() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let mut var1 = Variable::new(&spec);
    let set1 = var1.set_mut();
    // test element values
    let elem0_in = 55u64;
    let elem1_in = 132u64;
    let elem2_in = 99u64;

    // add values to set1
    set1.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
    set1.insert(Variable::try_from(elem1_in).unwrap()).unwrap();
    set1.insert(Variable::try_from(elem2_in).unwrap()).unwrap();

    // create another variable with the same spec
    let mut var2 = Variable::new(&spec);
    let set2 = var2.set_mut();

    // set set2 equal to set1
    assert!(set2.set_equal_to(&set1).is_ok());

    // check that set2 has the same values as set1
    assert_eq!(set2.len(), 3);
    assert!(
        set2.contains(&Variable::try_from(elem0_in).unwrap())
            .unwrap()
    );
    assert!(
        set2.contains(&Variable::try_from(elem1_in).unwrap())
            .unwrap()
    );
    assert!(
        set2.contains(&Variable::try_from(elem2_in).unwrap())
            .unwrap()
    );
}

#[test]
fn set_equal_to_incompatible_specs() {
    let spec1 = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let mut var1 = Variable::new(&spec1);
    let set1 = var1.set_mut();
    // test element values
    let elem0_in = 55u64;

    // add element to set1
    set1.insert(Variable::try_from(elem0_in).unwrap()).unwrap();

    // create another variable with an incompatible spec
    let spec2 = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed) // different encoding
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    let mut var2 = Variable::new(&spec2);
    let set2 = var2.set_mut();

    // Attempt to set set2 equal to set1 should fail due to incompatible specs
    assert_eq!(
        set2.set_equal_to(&set1).unwrap_err(),
        SetEqualToError::SpecError(SpecError::IncompatibleSpec(
            "Set { value_spec: Integer { encoding: Signed, storage: B64 }, element_ordering: None }"
                .to_string(),
            "Set { value_spec: Integer { encoding: Unsigned, storage: B64 }, element_ordering: None }"
                .to_string()
        ))
    );
}

#[test]
fn set_partial_eq_and_hash() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(SetElementOrdering::Ordered)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);
    {
        let set1 = var1.set_mut();

        // test element values
        let elem0_in = 132u64;
        let elem1_in = 55u64;

        // add values to set1
        set1.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
        set1.insert(Variable::try_from(elem1_in).unwrap()).unwrap();

        // create another variable with the same spec
        let set2 = var2.set_mut();

        // add the same values to set2
        set2.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
        set2.insert(Variable::try_from(elem1_in).unwrap()).unwrap();

        // check that the sets are equal
        assert_eq!(set1, set2);
        let mut hasher1 = DefaultHasher::new();
        // check that the hash values of the sets are equal
        set1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        set2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
    // check that the variables are equal
    assert_eq!(var1, var2);
    // check that the hash values of the variables are equal
    {
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }
    // modify set2 by adding a new element
    let elem0_in = 99u64;
    var2.set_mut()
        .insert(Variable::try_from(elem0_in).unwrap())
        .unwrap();

    // check that the sets are not equal anymore
    let set1 = var1.set();
    let set2 = var2.set();
    assert_ne!(set1, set2);
    // check that the hash values of the sets are not equal anymore
    {
        let mut hasher1 = DefaultHasher::new();
        set1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        set2.hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }

    // check that the variables are not equal anymore
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
fn set_partial_ord() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(SetElementOrdering::Ordered)
        .build();
    let mut var1 = Variable::new(&spec);
    let mut var2 = Variable::new(&spec);
    let mut var3 = Variable::new(&spec);
    {
        let set1 = var1.set_mut();

        // test element values
        let elem0_in = 132u64;
        let elem1_in = 55u64;

        // add values to set1
        set1.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
        set1.insert(Variable::try_from(elem1_in).unwrap()).unwrap();

        // new smaller test element value one
        let elem0_in = 100u64;
        let elem1_in = 55u64;

        // add the new values to set2
        let set2 = var2.set_mut();
        set2.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
        set2.insert(Variable::try_from(elem1_in).unwrap()).unwrap();

        // check that the sets compare correctly
        assert!(set2 < set1);
        assert!(set1 > set2);

        // new smaller test element value two
        let elem0_in = 132u64;
        let elem1_in = 1u64;

        // add the new values to set3
        let set3 = var3.set_mut();
        set3.insert(Variable::try_from(elem0_in).unwrap()).unwrap();
        set3.insert(Variable::try_from(elem1_in).unwrap()).unwrap();

        // check that the sets compare correctly
        assert!(set3 < set1);
        assert!(set1 > set3);
        assert!(set2 > set3);
        assert!(set3 < set2);
    }
}

#[test]
fn set_display_and_debug() {
    let spec = SetSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Unsigned)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .set_storage(SetElementOrdering::Ordered)
        .build();
    let mut var = Variable::new(&spec);
    assert_eq!(var.set().to_string(), "Set {}");
    assert_eq!(format!("{:?}", var.set()), "Set {}");

    // add values
    let set_mut = var.set_mut();
    set_mut.insert(Variable::try_from(55u64).unwrap()).unwrap();
    set_mut.insert(Variable::try_from(132u64).unwrap()).unwrap();
    set_mut.insert(Variable::try_from(99u64).unwrap()).unwrap();

    // check display
    assert_eq!(var.set().to_string(), "Set {55, 99, 132}");
    // check debug output
    assert_eq!(format!("{:?}", var.set()), "Set {55, 99, 132}");
}
