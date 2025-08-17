use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::interval_spec_builder::IntervalSpecBuilder,
    primitive::Primitive,
    primitive_specs::interval_spec::IntervalType,
};

#[test]
fn interval_spec_builder_no_type() {
    let interval_spec = IntervalSpecBuilder::new().build();
    assert_eq!(
        *interval_spec.specification_type(),
        DataSpecType::PrimitiveCategory(crate::primitive_category::PrimitiveCategory::Interval)
    );
    assert_eq!(interval_spec.specification_level(), DataSpecLevel::Compare);
}

#[test]
fn interval_spec_builder_year_month_type() {
    let interval_spec = IntervalSpecBuilder::new()
        .set_interval_type(IntervalType::YearMonth)
        .build();
    match interval_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::YearMonthInterval(def) => match def {
                Some(value) => match value.spec().interval_type() {
                    Some(IntervalType::YearMonth) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(interval_spec.specification_level(), DataSpecLevel::Access);
}
#[test]
fn interval_spec_builder_day_time_type() {
    let interval_spec = IntervalSpecBuilder::new()
        .set_interval_type(IntervalType::DayTime)
        .build();
    match interval_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::DaySecondInterval(def) => match def {
                Some(value) => match value.spec().interval_type() {
                    Some(IntervalType::DayTime) => {}
                    _ => assert!(false),
                },
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(interval_spec.specification_level(), DataSpecLevel::Access);
}
