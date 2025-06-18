use crate::{
    accessors::collections::set::Set, data_spec::{DataSpecLevel, DataSpecType}, data_spec_builders::{integer_spec_builder::IntegerSpecBuilder, set_spec_builder::SetSpecBuilder}, primitive::Primitive, primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage}
};

#[test]
fn set_no_element_spec() {
    let spec = SetSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Set(def) => {
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
fn set_with_access_element_spec() {
    let element_spec = IntegerSpecBuilder::new()
        .set_encoding(IntegerEncoding::Signed)
        .set_storage(IntegerStorage::B64)
        .build();
    let spec = SetSpecBuilder::new().set_element_spec(element_spec).build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Set(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.element_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(def) => {
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
fn set_with_compare_element_spec() {
    let element_spec = IntegerSpecBuilder::new().build();
    let spec = SetSpecBuilder::new()
        .set_element_spec(element_spec.clone())
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Set(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.element_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(def) => {
                            assert!(def.is_none());
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
        DataSpecLevel::Compare => {}
        _ => assert!(false),
    }
}

#[test]
#[should_panic(expected = "SetSpecBuilder: Sets require element's that are ordered so that they compare and hash reliably.")]
fn ordered_set_with_unordered_element_spec() {
    let element_spec = SetSpecBuilder::new()
        .set_element_spec(IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B64)
            .build())
        .set_storage(crate::primitive_specs::set_spec::SetStorage::Unordered)
        .build();
    SetSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(crate::primitive_specs::set_spec::SetStorage::Ordered)
        .build();
}

#[test]
#[should_panic(expected = "SetSpecBuilder: Sets require element's that are ordered so that they compare and hash reliably.")]
fn unordered_set_with_unordered_element_spec() {
    let element_spec = SetSpecBuilder::new()
        .set_element_spec(IntegerSpecBuilder::new()
            .set_encoding(IntegerEncoding::Signed)
            .set_storage(IntegerStorage::B64)
            .build())
        .set_storage(crate::primitive_specs::set_spec::SetStorage::Unordered)
        .build();
    SetSpecBuilder::new()
        .set_element_spec(element_spec)
        .set_storage(crate::primitive_specs::set_spec::SetStorage::Unordered)
        .build();
}