use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::{
        integer_spec_builder::IntegerSpecBuilder, sequence_spec_builder::SequenceSpecBuilder,
    },
    primitive::Primitive,
};

#[test]
fn sequence_no_value_spec() {
    let spec = SequenceSpecBuilder::new().build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Sequence(def) => {
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
fn sequence_with_value_spec() {
    let spec = SequenceSpecBuilder::new()
        .set_value_spec(
            IntegerSpecBuilder::new()
                .set_encoding(crate::primitive_specs::integer_spec::IntegerEncoding::Signed)
                .set_storage(crate::primitive_specs::integer_spec::IntegerStorage::B64)
                .build(),
        )
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Sequence(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.value_spec().as_ref().unwrap().specification_type() {
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
fn sequence_with_compare_value_spec() {
    let spec = SequenceSpecBuilder::new()
        .set_value_spec(IntegerSpecBuilder::new().build())
        .build();
    match spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::Sequence(def) => {
                let spec = def.as_ref().unwrap().spec();
                match spec.value_spec().as_ref().unwrap().specification_type() {
                    DataSpecType::Primitive(primitive) => match primitive {
                        Primitive::Integer(..) => {}
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
