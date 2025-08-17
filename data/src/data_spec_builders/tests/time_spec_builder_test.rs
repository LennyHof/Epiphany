use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::time_spec_builder::TimeSpecBuilder,
    primitive::Primitive,
    primitive_specs::time_spec::TimeType,
};

#[test]
fn no_time_type() {
    let time_spec = TimeSpecBuilder::new().build();
    assert_eq!(
        *time_spec.specification_type(),
        DataSpecType::PrimitiveCategory(crate::primitive_category::PrimitiveCategory::Time)
    );
    assert_eq!(time_spec.specification_level(), DataSpecLevel::Compare);
}
#[test]
fn local_time_type() {
    let time_spec = TimeSpecBuilder::new()
        .set_time_type(TimeType::Local)
        .build();
    match time_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::LocalTime(def) => match def {
                Some(value) => match *value.spec().time_type() {
                    Some(TimeType::Local) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(time_spec.specification_level(), DataSpecLevel::Access);
}
#[test]
fn zoned_time_type() {
    let time_spec = TimeSpecBuilder::new()
        .set_time_type(TimeType::Zoned)
        .build();
    match time_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::ZonedTime(def) => match def {
                Some(value) => match *value.spec().time_type() {
                    Some(TimeType::Zoned) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(time_spec.specification_level(), DataSpecLevel::Access);
}
