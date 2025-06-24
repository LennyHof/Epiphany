use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, tuple_spec_builder::TupleSpecBuilder,
    },
    primitive::Primitive,
    primitive_specs::integer_spec::{IntegerEncoding, IntegerStorage},
};

#[test]
fn tuple_no_value_specs() {
    let spec = TupleSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Tuple(def) => {
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
fn tuple_with_access_value_spec() {
    let spec = TupleSpecBuilder::new()
        .add_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(IntegerEncoding::Signed)
                .set_storage(IntegerStorage::B64)
                .build(),
        )
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Tuple(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.value_specs().as_ref().unwrap()[0].specification_type() {
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
fn tuple_with_compare_value_spec() {
    let spec = TupleSpecBuilder::new()
        .add_value_spec(IntegerSpecBuilder::new().build())
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Tuple(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.value_specs().as_ref().unwrap()[0].specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(def) => {
                            assert!(!def.is_some());
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
