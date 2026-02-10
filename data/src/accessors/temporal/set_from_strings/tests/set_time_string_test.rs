use crate::{
    accessors::temporal::time::TimeError, data_spec_builders::time_spec_builder::TimeSpecBuilder,
    primitive_specs::time_spec::TimeType, variable,
};

#[test]
fn set_time_from_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();

    for time_str in [
        "12:34:56.789456123",
        "T12:34:56.789456123",
        "123456.789456123",
        "T123456.789456123",
        "12:34:56,789456123",
        "T12:34:56,789456123",
        "123456,789456123",
        "T123456,789456123",
    ] {
        assert!(time_mut.set_from_string(time_str).is_ok());

        let result = time_mut.time();
        assert!(result.is_ok());
        let (hour, minute, second, millisecond, microsecond, nanosecond) = result.unwrap();
        assert_eq!(hour, 12);
        assert_eq!(minute, 34);
        assert_eq!(second, 56);
        assert_eq!(millisecond, 789);
        assert_eq!(microsecond, 456);
        assert_eq!(nanosecond, 123);
    }
}

#[test]
fn set_time_from_string_with_second_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();

    for time_str in ["12:34:56", "T12:34:56", "123456", "T123456"] {
        assert!(time_mut.set_from_string(time_str).is_ok());

        let result = time_mut.time();
        assert!(result.is_ok());
        let (hour, minute, second, millisecond, microsecond, nanosecond) = result.unwrap();
        assert_eq!(hour, 12);
        assert_eq!(minute, 34);
        assert_eq!(second, 56);
        assert_eq!(millisecond, 0);
        assert_eq!(microsecond, 0);
        assert_eq!(nanosecond, 0);
    }
}

#[test]
fn set_time_from_string_with_millisecond_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();

    for time_str in ["12:34:56.789", "T12:34:56.789", "123456.789", "T123456.789"] {
        assert!(time_mut.set_from_string(time_str).is_ok());
        let result = time_mut.time();
        assert!(result.is_ok());
        let (hour, minute, second, millisecond, microsecond, nanosecond) = result.unwrap();
        assert_eq!(hour, 12);
        assert_eq!(minute, 34);
        assert_eq!(second, 56);
        assert_eq!(millisecond, 789);
        assert_eq!(microsecond, 0);
        assert_eq!(nanosecond, 0);
    }

    for time_str in [
        "12:34:56.78",
        "T12:34:56.78",
        "123456.78",
        "T123456.78",
        "12:34:56,78",
        "T12:34:56,78",
        "123456,78",
        "T123456,78",
    ] {
        assert!(time_mut.set_from_string(time_str).is_ok());
        let result = time_mut.time();
        assert!(result.is_ok());
        let (hour, minute, second, millisecond, microsecond, nanosecond) = result.unwrap();
        assert_eq!(hour, 12);
        assert_eq!(minute, 34);
        assert_eq!(second, 56);
        assert_eq!(millisecond, 780);
        assert_eq!(microsecond, 0);
        assert_eq!(nanosecond, 0);
    }

    for time_str in [
        "12:34:56.7",
        "T12:34:56.7",
        "123456.7",
        "T123456.7",
        "12:34:56,7",
        "T12:34:56,7",
        "123456,7",
        "T123456,7",
    ] {
        assert!(time_mut.set_from_string(time_str).is_ok());
        let result = time_mut.time();
        assert!(result.is_ok());
        let (hour, minute, second, millisecond, microsecond, nanosecond) = result.unwrap();
        assert_eq!(hour, 12);
        assert_eq!(minute, 34);
        assert_eq!(second, 56);
        assert_eq!(millisecond, 700);
        assert_eq!(microsecond, 0);
        assert_eq!(nanosecond, 0);
    }
}

#[test]
fn invalid_hour_in_time_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();

    for time_str in [
        "2X:34:56.789456123",
        "T2X:34:56.789456123",
        "2X3456.789456123",
        "T2X3456.789456123",
        "2X:34:56,789456123",
        "T2X:34:56,789456123",
        "2X3456,789456123",
        "T2X3456,789456123",
    ] {
        let result = time_mut.set_from_string(time_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimeError::InvalidFormat(msg) => assert_eq!(
                msg,
                format!(
                    "Hour '2X' is not a valid hour number in time string '{}'",
                    time_str
                )
            ),
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn invalid_minute_in_time_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();
    for time_str in [
        "12:3X:56.789456123",
        "T12:3X:56.789456123",
        "123X56.789456123",
        "T123X56.789456123",
        "12:3X:56,789456123",
        "T12:3X:56,789456123",
        "123X56,789456123",
        "T123X56,789456123",
    ] {
        let result = time_mut.set_from_string(time_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimeError::InvalidFormat(msg) => assert_eq!(
                msg,
                format!(
                    "Minute '3X' is not a valid minute number in time string '{}'",
                    time_str
                )
            ),
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn invalid_second_in_time_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();

    for time_str in [
        "12:34:5X.789456123",
        "T12:34:5X.789456123",
        "12345X.789456123",
        "T12345X.789456123",
        "12:34:5X,789456123",
        "T12:34:5X,789456123",
        "12345X,789456123",
        "T12345X,789456123",
    ] {
        let result = time_mut.set_from_string(time_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimeError::InvalidFormat(msg) => assert_eq!(
                msg,
                format!(
                    "Second '5X' is not a valid second number in time string '{}'",
                    time_str
                )
            ),
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn invalid_fraction_in_time_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();
    for time_str in [
        "T12:34:56.78X456123",
        "12:34:56.78X456123",
        "123456.78X456123",
        "T123456.78X456123",
        "T12:34:56,78X456123",
        "12:34:56,78X456123",
        "123456,78X456123",
        "T123456,78X456123",
    ] {
        let result = time_mut.set_from_string(time_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimeError::InvalidFormat(msg) => assert_eq!(
                msg,
                format!(
                    "Second fraction '78X456123' is not a valid second fraction in time string '{}'",
                    time_str
                )
            ),
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn too_many_fraction_digits_in_time_string() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_mut = var.time_mut();
    for time_str in [
        "T12:34:56.7894561234",
        "12:34:56.7894561234",
        "123456.7894561234",
        "T123456.7894561234",
        "T12:34:56,7894561234",
        "12:34:56,7894561234",
        "123456,7894561234",
        "T123456,7894561234",
    ] {
        let result = time_mut.set_from_string(time_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            TimeError::InvalidFormat(msg) => assert_eq!(
                msg,
                format!(
                    "Second fraction '7894561234' is has too many digits in time string '{}'",
                    time_str
                )
            ),
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}
