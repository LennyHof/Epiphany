use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::date_time_spec_builder::DateTimeSpecBuilder,
    primitive::Primitive,
    primitive_specs::date_time_spec::DateTimeType,
};

#[test]
fn no_date_time_type() {
    let date_time_spec = DateTimeSpecBuilder::new().build();
    assert_eq!(
        *date_time_spec.specification_type(),
        DataSpecType::PrimitiveCategory(crate::primitive_category::PrimitiveCategory::DateTime)
    );
    assert_eq!(date_time_spec.specification_level(), DataSpecLevel::Compare);
}
#[test]
fn local_date_time_type() {
    let date_time_spec = DateTimeSpecBuilder::new()
        .set_date_time_type(DateTimeType::Local)
        .build();
    match date_time_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::LocalDateTime(def) => match def {
                Some(value) => match value.spec().date_time_type() {
                    Some(DateTimeType::Local) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(date_time_spec.specification_level(), DataSpecLevel::Access);
}
#[test]
fn zoned_date_time_type() {
    let date_time_spec = DateTimeSpecBuilder::new()
        .set_date_time_type(DateTimeType::Zoned)
        .build();
    match date_time_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::ZonedDateTime(def) => match def {
                Some(value) => match value.spec().date_time_type() {
                    Some(DateTimeType::Zoned) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(date_time_spec.specification_level(), DataSpecLevel::Access);
}
