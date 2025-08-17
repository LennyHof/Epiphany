use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::duration_spec_builder::DurationSpecBuilder,
    primitive::Primitive,
    primitive_specs::duration_spec::DurationType,
};

#[test]
fn duration_spec_builder_no_type() {
    let duration_spec = DurationSpecBuilder::new().build();
    assert_eq!(
        *duration_spec.specification_type(),
        DataSpecType::PrimitiveCategory(crate::primitive_category::PrimitiveCategory::Duration)
    );
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Compare);
}

#[test]
fn duration_spec_builder_year_month_type() {
    let duration_spec = DurationSpecBuilder::new()
        .set_duration_type(DurationType::YearMonth)
        .build();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::YearMonthDuration(def) => match def {
                Some(value) => match value.spec().duration_type() {
                    Some(DurationType::YearMonth) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}
#[test]
fn duration_spec_builder_day_time_type() {
    let duration_spec = DurationSpecBuilder::new()
        .set_duration_type(DurationType::DayTime)
        .build();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::DaySecondDuration(def) => match def {
                Some(value) => match value.spec().duration_type() {
                    Some(DurationType::DayTime) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}
