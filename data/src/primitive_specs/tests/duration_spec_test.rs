use crate::primitive_specs::duration_spec::{DurationResolution, DurationSpec, DurationType};
use crate::spec_compatibility::SpecCompatibility;

#[test]
fn duration_type_display() {
    assert_eq!(DurationType::YearToMonth.to_string(), "YearToMonth");
    assert_eq!(DurationType::DayToSecond.to_string(), "DayToSecond");
}

#[test]
fn duration_resolution_display() {
    assert_eq!(DurationResolution::Year.to_string(), "Year");
    assert_eq!(DurationResolution::Month.to_string(), "Month");
    assert_eq!(DurationResolution::Day.to_string(), "Day");
    assert_eq!(DurationResolution::Hour.to_string(), "Hour");
    assert_eq!(DurationResolution::Minute.to_string(), "Minute");
    assert_eq!(DurationResolution::Second.to_string(), "Second");
    assert_eq!(DurationResolution::Millisecond.to_string(), "Millisecond");
    assert_eq!(DurationResolution::Microsecond.to_string(), "Microsecond");
    assert_eq!(DurationResolution::Nanosecond.to_string(), "Nanosecond");
}

#[test]
fn duration_spec_no_type() {
    let spec = DurationSpec::new(None, None);
    assert!(spec.duration_type().is_none());
    assert!(spec.is_compatible_with(&DurationSpec::new(None, None)));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearToMonth), None)));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayToSecond), None)));
}

#[test]
fn duration_spec_year_month_type() {
    let spec = DurationSpec::new(Some(DurationType::YearToMonth), None);
    assert_eq!(*spec.duration_type(), Some(DurationType::YearToMonth));
    assert!(spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearToMonth), None)));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayToSecond), None)));
    assert!(spec.is_compatible_with(&DurationSpec::new(None, None)));
}

#[test]
fn duration_spec_day_time_type() {
    let spec = DurationSpec::new(Some(DurationType::DayToSecond), None);
    assert_eq!(*spec.duration_type(), Some(DurationType::DayToSecond));
    assert!(spec.is_compatible_with(&DurationSpec::new(Some(DurationType::DayToSecond), None)));
    assert!(!spec.is_compatible_with(&DurationSpec::new(Some(DurationType::YearToMonth), None)));
    assert!(spec.is_compatible_with(&DurationSpec::new(None, None)));
}

#[test]
fn duration_spec_display() {
    {
        let spec = DurationSpec::new(None, None);
        assert_eq!(
            spec.to_string(),
            "Duration { type: None, resolution: None }"
        );
    }

    {
        let spec = DurationSpec::new(Some(DurationType::YearToMonth), None);
        assert_eq!(
            spec.to_string(),
            "Duration { type: YearToMonth, resolution: None }"
        );
    }
    {
        let spec = DurationSpec::new(
            Some(DurationType::YearToMonth),
            Some(DurationResolution::Month),
        );
        assert_eq!(
            spec.to_string(),
            "Duration { type: YearToMonth, resolution: Month }"
        );
    }
    {
        let spec = DurationSpec::new(Some(DurationType::DayToSecond), None);
        assert_eq!(
            spec.to_string(),
            "Duration { type: DayToSecond, resolution: None }"
        );
    }
    {
        let spec = DurationSpec::new(
            Some(DurationType::DayToSecond),
            Some(DurationResolution::Second),
        );
        assert_eq!(
            spec.to_string(),
            "Duration { type: DayToSecond, resolution: Second }"
        );
    }
}

#[test]
fn duration_spec_debug() {
    let spec = DurationSpec::new(
        Some(DurationType::YearToMonth),
        Some(DurationResolution::Month),
    );
    assert_eq!(
        format!("{:?}", spec),
        "DurationSpec { duration_type: Some(YearToMonth), resolution: Some(Month) }"
    );
}
