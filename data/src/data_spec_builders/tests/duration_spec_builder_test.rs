use crate::{
    data_spec::{DataSpecLevel, DataSpecType},
    data_spec_builders::duration_spec_builder::DurationSpecBuilder,
    primitive::Primitive,
    primitive_specs::duration_spec::DurationType,
};

#[test]
fn no_type_no_resolution() {
    let duration_spec = DurationSpecBuilder::new().build().unwrap();
    assert_eq!(
        *duration_spec.specification_type(),
        DataSpecType::PrimitiveCategory(crate::primitive_category::PrimitiveCategory::Duration)
    );
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Compare);
}

#[test]
fn year_month_type_no_resolution() {
    let duration_spec = DurationSpecBuilder::new()
        .set_type(DurationType::YearToMonth)
        .build()
        .unwrap();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::YearToMonthDuration(def) => match def {
                Some(value) => {
                    match value.spec().duration_type() {
                        Some(DurationType::YearToMonth) => {}
                        _ => assert!(false),
                    }
                    match value.spec().resolution() {
                        None => {}
                        _ => assert!(false),
                    }
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}

#[test]
fn year_month_type_year_resolution() {
    let duration_spec = DurationSpecBuilder::new()
        .set_type(DurationType::YearToMonth)
        .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Year)
        .build()
        .unwrap();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::YearToMonthDuration(def) => match def {
                Some(value) => {
                    match value.spec().duration_type() {
                        Some(DurationType::YearToMonth) => {}
                        _ => assert!(false),
                    }
                    match value.spec().resolution() {
                        Some(crate::primitive_specs::duration_spec::DurationResolution::Year) => {}
                        _ => assert!(false),
                    }
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}

#[test]
fn day_time_type_no_resolution() {
    let duration_spec = DurationSpecBuilder::new()
        .set_type(DurationType::DayToSecond)
        .build()
        .unwrap();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => match primitive {
            Primitive::DayToSecondDuration(def) => match def {
                Some(value) => {
                    match value.spec().duration_type() {
                        Some(DurationType::DayToSecond) => {}
                        _ => assert!(false),
                    }
                    match value.spec().resolution() {
                        None => {}
                        _ => assert!(false),
                    }
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        },
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}

#[test]
fn day_time_type_millisecond_resolution() {
    let duration_spec = DurationSpecBuilder::new()
        .set_type(DurationType::DayToSecond)
        .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Millisecond)
        .build()
        .unwrap();
    match duration_spec.specification_type() {
        DataSpecType::Primitive(primitive) => {
            match primitive {
                Primitive::DayToSecondDuration(def) => match def {
                    Some(value) => {
                        match value.spec().duration_type() {
                            Some(DurationType::DayToSecond) => {}
                            _ => assert!(false),
                        }
                        match value.spec().resolution() {
                        Some(crate::primitive_specs::duration_spec::DurationResolution::Millisecond) => {}
                        _ => assert!(false),
                    }
                    }
                    _ => assert!(false),
                },
                _ => assert!(false),
            }
        }
        _ => assert!(false),
    }
    assert_eq!(duration_spec.specification_level(), DataSpecLevel::Access);
}

#[test]
fn no_type_with_resolution() {
    let result = DurationSpecBuilder::new()
        .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Second)
        .build();
    assert!(result.is_err());
    match result.err() {
        Some(e) => {
            assert_eq!(
                e.to_string(),
                "Cannot set resolution without setting duration type."
            );
        }
        None => assert!(false),
    }
}

#[test]
fn year_month_type_incompatible_resolution() {
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Day)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Day) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Hour)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Hour) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Minute)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Minute) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Second)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Second) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Millisecond)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Millisecond) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Microsecond)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Microsecond) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Nanosecond)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Nanosecond) for duration type (YearToMonth)."
                );
            }
            None => assert!(false),
        }
    }
}

#[test]
fn day_time_type_incompatible_resolution() {
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::DayToSecond)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Year)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Year) for duration type (DayToSecond)."
                );
            }
            None => assert!(false),
        }
    }
    {
        let result = DurationSpecBuilder::new()
            .set_type(DurationType::DayToSecond)
            .set_resolution(crate::primitive_specs::duration_spec::DurationResolution::Month)
            .build();
        assert!(result.is_err());
        match result.err() {
            Some(e) => {
                assert_eq!(
                    e.to_string(),
                    "Incompatible resolution (Month) for duration type (DayToSecond)."
                );
            }
            None => assert!(false),
        }
    }
}
