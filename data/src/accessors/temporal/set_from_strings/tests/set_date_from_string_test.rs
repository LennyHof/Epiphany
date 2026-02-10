use crate::{
    accessors::temporal::date::DateError, data_spec_builders::date_spec_builder::DateSpecBuilder,
    variable,
};

#[test]
fn set_date_from_string() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();

    for date_str in ["2024-06-15", "2024-6-15", "20240615"] {
        let result = date_mut.set_from_string(date_str);
        assert!(result.is_ok());

        let (year, month, day) = date_mut.date().unwrap();
        assert_eq!(year, 2024);
        assert_eq!(month, 6);
        assert_eq!(day, 15);
    }
}

#[test]
fn invalid_year_in_date_string() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();

    for date_str in ["2I24-13-15", "2I24-6-15", "2I240615"] {
        let result = date_mut.set_from_string(date_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            DateError::InvalidFormat(mes) => {
                assert_eq!(
                    mes,
                    format!(
                        "Year '2I24' is not a valid year number in date string '{}'",
                        date_str
                    )
                );
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn invalid_month_in_date_string() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();

    for date_str in ["2024-I3-15", "2024-I3-15", "2024I315"] {
        let result = date_mut.set_from_string(date_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            DateError::InvalidFormat(mes) => {
                assert_eq!(
                    mes,
                    format!(
                        "Month 'I3' is not a valid month number in date string '{}'",
                        date_str
                    )
                );
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn invalid_day_in_date_string() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();

    for date_str in ["2024-06-1A", "2024061A"] {
        let result = date_mut.set_from_string(date_str);
        assert!(result.is_err());
        match result.err().unwrap() {
            DateError::InvalidFormat(mes) => {
                assert_eq!(
                    mes,
                    format!(
                        "Day '1A' is not a valid day number in date string '{}'",
                        date_str
                    )
                );
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }
}

#[test]
fn set_date_from_string_invalid_format() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let date_str = "2024/06/15"; // Invalid format

    let result = date_mut.set_from_string(date_str);
    assert!(result.is_err());
    match result.err().unwrap() {
        DateError::InvalidFormat(mes) => {
            assert_eq!(
                mes,
                "Date string '2024/06/15' is not in a valid ISO 8601 format  (YYYY-MM-DD), or (YYYYMMDD)"
            );
        }
        _ => panic!("Expected InvalidFormat error"),
    }
}
