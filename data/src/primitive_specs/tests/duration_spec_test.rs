use crate::primitive_specs::duration_spec::{DurationSpec, DurationType};
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn duration_spec_no_type() {
    let spec = DurationSpec::new(None);
    assert!(spec.duration_type().is_none());
    assert!(spec.is_compatible_with(&DurationSpec::new(None)));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearMonth))));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayTime))));
}
#[test]
fn duration_spec_year_month_type() {
    let spec = DurationSpec::new(Some(DurationType::YearMonth));
    assert_eq!(*spec.duration_type(), Some(DurationType::YearMonth));
    assert!(spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearMonth))));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayTime))));
    assert!(spec.is_compatible_with(&DurationSpec::new(None)));
}
#[test]
fn duration_spec_day_time_type() {
    let spec = DurationSpec::new(Some(DurationType::DayTime));
    assert_eq!(*spec.duration_type(), Some(DurationType::DayTime));
    assert!(spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayTime))));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearMonth))));
    assert!(spec.is_compatible_with(&DurationSpec::new(None)));
}
#[test]
fn duration_type_to_string() {
    {
        let spec = DurationSpec::new(Some(DurationType::YearMonth));
        assert_eq!(spec.duration_type().unwrap().to_string(), "YearMonth");
        assert_eq!(spec.to_string(), "Duration { type: YearMonth }");
    }
    {
        let spec = DurationSpec::new(Some(DurationType::DayTime));
        assert_eq!(spec.duration_type().unwrap().to_string(), "DayTime");
        assert_eq!(spec.to_string(), "Duration { type: DayTime }");
    }
}