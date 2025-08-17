use crate::primitive_specs::interval_spec::{IntervalSpec, IntervalType};
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn interval_spec_no_type() {
    let spec = IntervalSpec::new(None);
    assert!(spec.interval_type().is_none());
    assert!(spec.is_compatible_with(&IntervalSpec::new(None)));
    assert!(!spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::YearMonth))));
    assert!(!spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::DayTime))));
}
#[test]
fn interval_spec_year_month_type() {
    let spec = IntervalSpec::new(Some(IntervalType::YearMonth));
    assert_eq!(*spec.interval_type(), Some(IntervalType::YearMonth));
    assert!(spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::YearMonth))));
    assert!(!spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::DayTime))));
    assert!(spec.is_compatible_with(&IntervalSpec::new(None)));
}
#[test]
fn interval_spec_day_time_type() {
    let spec = IntervalSpec::new(Some(IntervalType::DayTime));
    assert_eq!(*spec.interval_type(), Some(IntervalType::DayTime));
    assert!(spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::DayTime))));
    assert!(!spec.is_compatible_with(&IntervalSpec::new(Some(IntervalType::YearMonth))));
    assert!(spec.is_compatible_with(&IntervalSpec::new(None)));
}
#[test]
fn interval_type_to_string() {
    {
        let spec = IntervalSpec::new(Some(IntervalType::YearMonth));
        assert_eq!(spec.interval_type().unwrap().to_string(), "YearMonth");
        assert_eq!(spec.to_string(), "Interval { type: YearMonth }");
    }
    {
        let spec = IntervalSpec::new(Some(IntervalType::DayTime));
        assert_eq!(spec.interval_type().unwrap().to_string(), "DayTime");
        assert_eq!(spec.to_string(), "Interval { type: DayTime }");
    }
}