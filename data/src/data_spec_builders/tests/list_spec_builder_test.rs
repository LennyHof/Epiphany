use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::{
        boolean_spec_builder::BooleanSpecBuilder, integer_spec_builder::IntegerSpecBuilder,
        list_spec_builder::ListSpecBuilder,
    },
    primitive::Primitive,
    primitive_specs::list_spec::ListStorage,
};

#[test]
fn list_storage_display() {
    assert_eq!(ListStorage::FixedSize(10).to_string(), "FixedSize(10)");
    assert_eq!(
        ListStorage::FixedCapacity(10).to_string(),
        "FixedCapacity(10)"
    );
    assert_eq!(
        ListStorage::InitialCapacity(10).to_string(),
        "InitialCapacity(10)"
    );
    assert_eq!(ListStorage::VariableSize.to_string(), "VariableSize");
}

#[test]
fn list_spec_builder_default() {
    let spec = ListSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                assert!(def.is_none());
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn list_no_element_spec() {
    let spec = ListSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                assert!(def.is_none());
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_element_spec() {
    let element_spec = BooleanSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert!(spec.storage().is_none());
                match spec.element_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Boolean(def) => {
                            assert!(def.is_some());
                        }
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_fixed_size() {
    let element_spec = BooleanSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(ListStorage::FixedSize(10))
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert_eq!(spec.storage().unwrap(), ListStorage::FixedSize(10));
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_fixed_capacity() {
    let element_spec = BooleanSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(ListStorage::FixedCapacity(10))
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert_eq!(spec.storage().unwrap(), ListStorage::FixedCapacity(10));
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_initial_capacity() {
    let element_spec = BooleanSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(ListStorage::InitialCapacity(10))
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert_eq!(spec.storage().unwrap(), ListStorage::InitialCapacity(10));
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_variable_size() {
    let element_spec = BooleanSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(ListStorage::VariableSize)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert_eq!(spec.storage().unwrap(), ListStorage::VariableSize);
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Access => {}
        _ => assert!(false),
    }
}

#[test]
fn list_with_compare_element_spec() {
    let element_spec = IntegerSpecBuilder::new().build();
    let spec = ListSpecBuilder::new()
        .set_element_spec(element_spec)
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::List(def) => {
                let spec = def.as_ref().unwrap().spec();
                assert!(spec.storage().is_none());
                match spec.element_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(_) => {}
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    match spec.specification_level() {
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
#[should_panic(expected = "ListSpecBuilder: storage is set but no element spec is set.")]
fn list_with_storage_but_no_element_spec() {
    ListSpecBuilder::new()
        .set_storage(ListStorage::FixedSize(10))
        .build();
}
