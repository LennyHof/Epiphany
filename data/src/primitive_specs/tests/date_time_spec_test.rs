use crate::primitive_specs::date_time_spec::{DateTimeSpec, DateTimeType};
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn date_time_no_type() {
    let spec = DateTimeSpec::new(None);
    assert!(spec.date_time_type().is_none());
    assert!(spec.is_compatible_with(&DateTimeSpec::new(None)));
    assert!(!spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Local))));
    assert!(!spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Zoned))));
}
#[test]
fn date_time_local_type() {
    let spec = DateTimeSpec::new(Some(DateTimeType::Local));
    assert_eq!(*spec.date_time_type(), Some(DateTimeType::Local));
    assert!(spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Local))));
    assert!(!spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Zoned))));
    assert!(spec.is_compatible_with(&DateTimeSpec::new(None)));
}
#[test]
fn date_time_zoned_type() {
    let spec = DateTimeSpec::new(Some(DateTimeType::Zoned));
    assert_eq!(*spec.date_time_type(), Some(DateTimeType::Zoned));
    assert!(spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Zoned))));
    assert!(!spec.is_compatible_with(&DateTimeSpec::new(Some(DateTimeType::Local))));
    assert!(spec.is_compatible_with(&DateTimeSpec::new(None)));
}
#[test]
fn date_time_type_to_string() {
    {
        let spec = DateTimeSpec::new(Some(DateTimeType::Local));
        assert_eq!(spec.date_time_type().unwrap().to_string(), "Local");
        assert_eq!(spec.to_string(), "DateTime { type: Local }");
    }
    {
        let spec = DateTimeSpec::new(Some(DateTimeType::Zoned));
        assert_eq!(spec.date_time_type().unwrap().to_string(), "Zoned");
        assert_eq!(spec.to_string(), "DateTime { type: Zoned }");
    }
}
