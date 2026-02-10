use crate::{
    data_spec_builders::{
        float_spec_builder::FloatSpecBuilder, integer_spec_builder::IntegerSpecBuilder,
        tuple_spec_builder::TupleSpecBuilder,
    },
    primitive_specs::{
        float_spec::FloatStorage,
        integer_spec::{IntegerEncoding, IntegerStorage},
    },
    set_equal_to::{SetEqualTo, SetEqualToError},
    spec_compatibility::SpecError,
    variable::Variable,
};

#[test]
fn tuple() {
    let mut var = Variable::new(
        &TupleSpecBuilder::new()
            .add_value_spec(
                IntegerSpecBuilder::new()
                    .set_encoding(IntegerEncoding::Signed)
                    .set_storage(IntegerStorage::B64)
                    .build(),
            )
            .add_value_spec(
                FloatSpecBuilder::new()
                    .set_storage(FloatStorage::B64)
                    .build(),
            )
            .build(),
    );

    // Verify source initial values
    assert_eq!(var.tuple().get(0).unwrap().integer().i64().unwrap(), 0i64);
    assert_eq!(var.tuple().get(1).unwrap().float().f64().unwrap(), 0f64);

    // Set values in the tuple variable
    // This will use the tuple_mut() method to get a mutable reference to the tuple
    {
        let tuple_mut = var.tuple_mut();
        tuple_mut
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }

    // Verify values after setting
    // This will use the tuple() method to get an immutable reference to the tuple
    assert_eq!(var.tuple().get(0).unwrap().integer().i64().unwrap(), 42i64);
    assert_eq!(var.tuple().get(1).unwrap().float().f64().unwrap(), 3.14f64);
}

#[test]
fn tuple_set_equal_to() {
    let tuple_spec = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let mut source = Variable::new(&tuple_spec);

    // Set values in the source variable
    {
        let tuple_mut = source.tuple_mut();
        tuple_mut
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }
    // Verify source values
    assert_eq!(
        source.tuple().get(0).unwrap().integer().i64().unwrap(),
        42i64
    );
    assert_eq!(
        source.tuple().get(1).unwrap().float().f64().unwrap(),
        3.14f64
    );

    // Create a target variable and set it equal to the source
    // This will copy the values from source to target
    let mut target = Variable::new(&tuple_spec);
    target.set_equal_to(&source).unwrap();

    // Verify target values after copying
    assert_eq!(
        target.tuple().get(0).unwrap().integer().i64().unwrap(),
        42i64
    );
    assert_eq!(
        target.tuple().get(1).unwrap().float().f64().unwrap(),
        3.14f64
    );
}

#[test]
fn tuple_set_equal_to_different_types() {
    let tuple_spec1 = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let tuple_spec2 = TupleSpecBuilder::new()
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let mut source = Variable::new(&tuple_spec1);

    // Set values in the source variable
    {
        let tuple_mut = source.tuple_mut();
        tuple_mut
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }

    // Create a target variable with different types
    let mut target = Variable::new(&tuple_spec2);

    // Attempt to set target equal to source, which should fail due to type mismatch
    // The target tuple only has a Float value spec, while the source has both Integer and Float
    // This should raise a SetEqualToError due to incompatible specs
    assert_eq!(
        target.set_equal_to(&source).unwrap_err(),
        SetEqualToError::SpecError(SpecError::IncompatibleSpec(
            "Tuple { value_specs: {Float { storage: B64 }} }"
                .to_string(),
            "Tuple { value_specs: {Integer { encoding: Signed, storage: B64 }, Float { storage: B64 }} }"
                .to_string()
        ))
    );
}

#[test]
fn tuple_partial_eq_and_hash() {
    let tuple_spec = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let mut var1 = Variable::new(&tuple_spec);
    let mut var2 = Variable::new(&tuple_spec);

    // Set values in both variables
    {
        let tuple_mut1 = var1.tuple_mut();
        tuple_mut1
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut1
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();

        let tuple_mut2 = var2.tuple_mut();
        tuple_mut2
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut2
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }

    // Check equality
    assert_eq!(var1, var2);
    assert!(var1.eq(&var2));
    assert_eq!(var1.tuple(), var2.tuple());
    assert!(var1.tuple().eq(&var2.tuple()));

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    {
        // Check hash of tuples
        let mut hasher1 = DefaultHasher::new();
        var1.tuple().hash(&mut hasher1);

        let mut hasher2 = DefaultHasher::new();
        var2.tuple().hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    {
        // Check hash of variables
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        assert_eq!(hasher1.finish(), hasher2.finish());
    }

    // Change a value in var2 and check inequality
    {
        let tuple_mut2 = var2.tuple_mut();
        tuple_mut2
            .set(0, Variable::try_from(43i64).unwrap())
            .unwrap();
    }
    assert_ne!(var1, var2);
    assert!(!var1.eq(&var2));
    assert_ne!(var1.tuple(), var2.tuple());
    assert!(!var1.tuple().eq(&var2.tuple()));

    {
        // Check hash of tuples after modification
        let mut hasher1 = DefaultHasher::new();
        var1.tuple().hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.tuple().hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
    {
        // Check hash of variables after modification
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        assert_ne!(hasher1.finish(), hasher2.finish());
    }
}

#[test]
fn tuple_partial_ord() {
    let tuple_spec = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let mut var1 = Variable::new(&tuple_spec);
    let mut var2 = Variable::new(&tuple_spec);

    // Set values in both variables
    {
        let tuple_mut1 = var1.tuple_mut();
        tuple_mut1
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut1
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();

        let tuple_mut2 = var2.tuple_mut();
        tuple_mut2
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut2
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }

    // Check partial ordering
    assert_eq!(var1.partial_cmp(&var2), Some(std::cmp::Ordering::Equal));
    assert_eq!(
        var1.tuple().partial_cmp(var2.tuple()),
        Some(std::cmp::Ordering::Equal)
    );

    // Change a value in var2 and check ordering
    {
        let tuple_mut2 = var2.tuple_mut();
        tuple_mut2
            .set(0, Variable::try_from(43i64).unwrap())
            .unwrap();
    }
    assert_eq!(var1.partial_cmp(&var2), Some(std::cmp::Ordering::Less));
    assert_eq!(
        var1.tuple().partial_cmp(var2.tuple()),
        Some(std::cmp::Ordering::Less)
    );
}

#[test]
fn tuple_display_and_debug() {
    let tuple_spec = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .add_value_spec(
            FloatSpecBuilder::new()
                .set_storage(FloatStorage::B64)
                .build(),
        )
        .build();

    let mut var = Variable::new(&tuple_spec);

    // Set values in the tuple
    {
        let tuple_mut = var.tuple_mut();
        tuple_mut
            .set(0, Variable::try_from(42i64).unwrap())
            .unwrap();
        tuple_mut
            .set(1, Variable::try_from(3.14f64).unwrap())
            .unwrap();
    }

    // check display
    assert_eq!(var.tuple().to_string(), "Tuple {42, 3.14}");
    // check debug output
    assert_eq!(format!("{:?}", var.tuple()), "Tuple {42, 3.14}");
}
