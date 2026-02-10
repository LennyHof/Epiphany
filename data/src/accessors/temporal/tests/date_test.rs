use std::rc::Rc;

use crate::{
    accessors::temporal::date::{DateError, DayOfWeek, days_in_month, is_leap_year, month_name},
    adaptor::Adaptor,
    adaptors::temporal_adaptors::date_adaptor::DateAdaptor,
    data_spec_builders::date_spec_builder::DateSpecBuilder,
    primitive_specs::date_spec::DateSpec,
    set_equal_to::SetEqualTo,
    variable,
};

#[test]
fn is_leap_year_test() {
    assert_eq!(is_leap_year(2020), true);
    assert_eq!(is_leap_year(2021), false);
    assert_eq!(is_leap_year(1900), false);
    assert_eq!(is_leap_year(2000), true);
}

#[test]
fn month_name_test() {
    assert_eq!(month_name(1), "January");
    assert_eq!(month_name(2), "February");
    assert_eq!(month_name(3), "March");
    assert_eq!(month_name(4), "April");
    assert_eq!(month_name(5), "May");
    assert_eq!(month_name(6), "June");
    assert_eq!(month_name(7), "July");
    assert_eq!(month_name(8), "August");
    assert_eq!(month_name(9), "September");
    assert_eq!(month_name(10), "October");
    assert_eq!(month_name(11), "November");
    assert_eq!(month_name(12), "December");
}

#[test]
fn days_in_month_test_test() {
    assert_eq!(days_in_month(false, 1), 31);
    assert_eq!(days_in_month(true, 1), 31);
    assert_eq!(days_in_month(false, 2), 28);
    assert_eq!(days_in_month(true, 2), 29);
    assert_eq!(days_in_month(false, 3), 31);
    assert_eq!(days_in_month(true, 3), 31);
    assert_eq!(days_in_month(false, 4), 30);
    assert_eq!(days_in_month(true, 4), 30);
    assert_eq!(days_in_month(false, 5), 31);
    assert_eq!(days_in_month(true, 5), 31);
    assert_eq!(days_in_month(false, 6), 30);
    assert_eq!(days_in_month(true, 6), 30);
    assert_eq!(days_in_month(false, 7), 31);
    assert_eq!(days_in_month(true, 7), 31);
    assert_eq!(days_in_month(false, 8), 31);
    assert_eq!(days_in_month(true, 8), 31);
    assert_eq!(days_in_month(false, 9), 30);
    assert_eq!(days_in_month(true, 9), 30);
    assert_eq!(days_in_month(false, 10), 31);
    assert_eq!(days_in_month(true, 10), 31);
    assert_eq!(days_in_month(false, 11), 30);
    assert_eq!(days_in_month(true, 11), 30);
    assert_eq!(days_in_month(false, 12), 31);
    assert_eq!(days_in_month(true, 12), 31);
}

#[test]
fn default_date() {
    let var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date = var.date();
    let (year, month, day) = date.date().unwrap();
    assert_eq!(year, 1);
    assert_eq!(month, 1);
    assert_eq!(day, 1);
}

#[test]
fn non_leap_year_date() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2024, 3, 14).unwrap();
    let (year, month, day) = date_mut.date().unwrap();
    assert_eq!(year, 2024);
    assert_eq!(month, 3);
    assert_eq!(day, 14);
    assert_eq!(date_mut.year().unwrap(), 2024);
    assert_eq!(date_mut.month().unwrap(), 3);
    assert_eq!(date_mut.day().unwrap(), 14);
}

#[test]
fn leap_year_date() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2020, 2, 29).unwrap();
    let (year, month, day) = date_mut.date().unwrap();
    assert_eq!(year, 2020);
    assert_eq!(month, 2);
    assert_eq!(day, 29);
    assert_eq!(date_mut.year().unwrap(), 2020);
    assert_eq!(date_mut.month().unwrap(), 2);
    assert_eq!(date_mut.day().unwrap(), 29);
}

#[test]
fn day_of_year() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2021, 3, 1).unwrap();
    assert_eq!(date_mut.day_of_year().unwrap(), 60);

    date_mut.set_date(2020, 3, 1).unwrap();
    assert_eq!(date_mut.day_of_year().unwrap(), 61);

    date_mut.set_date(2021, 12, 31).unwrap();
    assert_eq!(date_mut.day_of_year().unwrap(), 365);

    date_mut.set_date(2020, 12, 31).unwrap();
    assert_eq!(date_mut.day_of_year().unwrap(), 366);
}

#[test]
fn day_of_week() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2021, 10, 5).unwrap(); // Tuesday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Tuesday);

    date_mut.set_date(2021, 10, 6).unwrap(); // Wednesday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Wednesday);

    date_mut.set_date(2021, 10, 7).unwrap(); // Thursday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Thursday);

    date_mut.set_date(2021, 10, 8).unwrap(); // Friday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Friday);

    date_mut.set_date(2021, 10, 9).unwrap(); // Saturday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Saturday);

    date_mut.set_date(2021, 10, 10).unwrap(); // Sunday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Sunday);

    date_mut.set_date(2021, 10, 11).unwrap(); // Monday
    assert_eq!(date_mut.day_of_week().unwrap(), DayOfWeek::Monday);
}

#[test]
fn day_of_week_display() {
    assert_eq!(format!("{}", DayOfWeek::Monday), "Monday");
    assert_eq!(format!("{}", DayOfWeek::Tuesday), "Tuesday");
    assert_eq!(format!("{}", DayOfWeek::Wednesday), "Wednesday");
    assert_eq!(format!("{}", DayOfWeek::Thursday), "Thursday");
    assert_eq!(format!("{}", DayOfWeek::Friday), "Friday");
    assert_eq!(format!("{}", DayOfWeek::Saturday), "Saturday");
    assert_eq!(format!("{}", DayOfWeek::Sunday), "Sunday");
}

#[test]
fn year_out_of_bounds_low() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(0, 5, 10);
    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            DateError::YearOutOfBounds(year) => {
                assert_eq!(year, 0);
            }
            _ => panic!("Expected YearOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Year value 0 is out of bounds (1 to 9999)."
        );
    }
}

#[test]
fn year_out_of_bounds_high() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(10000, 1, 1);
    assert!(result.is_err());
    if let Err(err) = result {
        match err {
            DateError::YearOutOfBounds(year) => {
                assert_eq!(year, 10000);
            }
            _ => panic!("Expected YearOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Year value 10000 is out of bounds (1 to 9999)."
        );
    }
}

#[test]
fn month_out_of_bounds_low() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(2021, 0, 15);
    assert!(result.is_err());
    if let Err(err) = result {
        match &err {
            DateError::MonthOutOfBounds(month) => {
                assert_eq!(*month, 0);
            }
            _ => panic!("Expected MonthOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Month value 0 is out of bounds (1 to 12)."
        );
    }
}

#[test]
fn month_out_of_bounds_high() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(2021, 13, 15);
    assert!(result.is_err());
    if let Err(err) = result {
        match &err {
            DateError::MonthOutOfBounds(month) => {
                assert_eq!(*month, 13);
            }
            _ => panic!("Expected MonthOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Month value 13 is out of bounds (1 to 12)."
        );
    }
}

#[test]
fn day_out_of_bounds_low() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(2021, 5, 0);
    assert!(result.is_err());
    if let Err(err) = result {
        match &err {
            DateError::DayOutOfBounds(day, month_name, days_in_month) => {
                assert_eq!(*day, 0);
                assert_eq!(*month_name, "May");
                assert_eq!(*days_in_month, 31);
            }
            _ => panic!("Expected DayOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Day value 0 is out of bounds for month May. Valid range is 1 to 31."
        );
    }
}

#[test]
fn day_out_of_bounds_high() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    let result = date_mut.set_date(2021, 2, 29);
    assert!(result.is_err());
    if let Err(err) = result {
        match &err {
            DateError::DayOutOfBounds(day, month_name, days_in_month) => {
                assert_eq!(*day, 29);
                assert_eq!(*month_name, "February");
                assert_eq!(*days_in_month, 28);
            }
            _ => panic!("Expected DayOutOfBounds error"),
        }
        assert_eq!(
            format!("{}", err),
            "Day value 29 is out of bounds for month February. Valid range is 1 to 28."
        );
    }
}

#[test]
fn date_display() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2022, 7, 20).unwrap();
    assert_eq!(format!("{}", date_mut), "2022-07-20");
}

#[test]
fn date_debug() {
    let mut var = variable::Variable::new(&DateSpecBuilder::new().build());
    let date_mut = var.date_mut();
    date_mut.set_date(2022, 7, 20).unwrap();
    assert_eq!(
        format!("{:?}", date_mut),
        "Date { year: 2022, month: 7, day: 20 }"
    );
}

#[test]
fn date_set_equal_to() {
    // set value to be copied
    let mut var1 = variable::Variable::new(&DateSpecBuilder::new().build());
    let date1_mut = var1.date_mut();
    date1_mut.set_date(2022, 7, 20).unwrap();

    // test set equal to via accessor
    let mut var2 = variable::Variable::new(&DateSpecBuilder::new().build());
    let date2_mut = var2.date_mut();
    date2_mut.set_equal_to(&date1_mut).unwrap();
    assert_eq!(date2_mut.date().unwrap(), (2022, 7, 20));

    // test set equal to via variable
    let mut var3 = variable::Variable::new(&DateSpecBuilder::new().build());
    var3.set_equal_to(&var1).unwrap();
    assert_eq!(var3.date().date().unwrap(), (2022, 7, 20));
}

#[test]
fn date_partial_eq_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut var1 = variable::Variable::new(&DateSpecBuilder::new().build());
    let mut var2 = variable::Variable::new(&DateSpecBuilder::new().build());
    let mut var3 = variable::Variable::new(&DateSpecBuilder::new().build());
    {
        let date1_mut = var1.date_mut();
        date1_mut.set_date(2022, 11, 5).unwrap();

        let date2_mut = var2.date_mut();
        date2_mut.set_date(2022, 11, 5).unwrap();

        let date3_mut = var3.date_mut();
        date3_mut.set_date(2021, 10, 4).unwrap();
    }

    // test accessor equality
    assert_eq!(var1.date(), var2.date());
    assert_ne!(var1.date(), var3.date());

    // test variable equality
    assert_eq!(var1, var2);
    assert_ne!(var1, var3);

    // test accessor hashing
    {
        let mut hasher1 = DefaultHasher::new();
        var1.date().hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        var2.date().hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        var3.date().hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    // test variable hashing
    {
        let mut hasher1 = DefaultHasher::new();
        var1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        var2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        var3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}

#[test]
fn date_ord() {
    let mut var1 = variable::Variable::new(&DateSpecBuilder::new().build());
    let mut var2 = variable::Variable::new(&DateSpecBuilder::new().build());
    let mut var3 = variable::Variable::new(&DateSpecBuilder::new().build());
    {
        let date1_mut = var1.date_mut();
        date1_mut.set_date(2022, 5, 15).unwrap();

        let date2_mut = var2.date_mut();
        date2_mut.set_date(2023, 1, 10).unwrap();

        let date3_mut = var3.date_mut();
        date3_mut.set_date(2022, 5, 15).unwrap();
    }

    // test accessor ordering
    let date1 = var1.date();
    let date2 = var2.date();
    let date3 = var3.date();
    assert!(date1 < date2);
    assert!(date2 > date1);
    assert!(date1 <= date3);
    assert!(date1 >= date3);

    // test variable ordering
    assert!(var1 < var2);
    assert!(var2 > var1);
    assert!(var1 <= var3);
    assert!(var1 >= var3);
    assert_eq!(var1.cmp(&var3), std::cmp::Ordering::Equal);
}

struct CustomDateAdaptor {
    spec: Rc<DateSpec>,
    year: u32,
    month: u32,
    day: u32,
}

impl CustomDateAdaptor {
    pub fn new(spec: Rc<DateSpec>) -> Self {
        Self {
            spec: spec.clone(),
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl Adaptor for CustomDateAdaptor {}

impl DateAdaptor for CustomDateAdaptor {
    fn spec(&self) -> &Rc<DateSpec> {
        &self.spec
    }

    fn stores_date_as_days(&self) -> bool {
        false
    }

    fn set_date(&mut self, year: u32, month: u32, day: u32) -> Result<(), DateError> {
        self.year = year;
        self.month = month;
        self.day = day;
        Ok(())
    }

    fn date(&self) -> Result<(u32, u32, u32), DateError> {
        Ok((self.year, self.month, self.day))
    }

    fn year(&self) -> Result<u32, DateError> {
        Ok(self.year)
    }

    fn month(&self) -> Result<u32, DateError> {
        Ok(self.month)
    }

    fn day(&self) -> Result<u32, DateError> {
        Ok(self.day)
    }
}

#[test]
fn custom_date_adaptor() {
    let date_spec = Rc::new(DateSpec::new());
    let custom_date = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor = crate::accessors::temporal::date::Date::new(Box::new(custom_date));
    date_accessor.set_date(2020, 3, 14).unwrap();
    assert_eq!(date_accessor.year().unwrap(), 2020);
    assert_eq!(date_accessor.month().unwrap(), 3);
    assert_eq!(date_accessor.day().unwrap(), 14);
}

#[test]
fn custom_date_adaptor_set_equal_to() {
    let date_spec = Rc::new(DateSpec::new());
    let custom_date1 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor1 = crate::accessors::temporal::date::Date::new(Box::new(custom_date1));
    date_accessor1.set_date(2021, 6, 18).unwrap();

    let custom_date2 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor2 = crate::accessors::temporal::date::Date::new(Box::new(custom_date2));
    date_accessor2.set_date(2019, 12, 31).unwrap();

    date_accessor2.set_equal_to(&date_accessor1).unwrap();

    assert_eq!(date_accessor2.year().unwrap(), 2021);
    assert_eq!(date_accessor2.month().unwrap(), 6);
    assert_eq!(date_accessor2.day().unwrap(), 18);
}

#[test]
fn custom_date_adaptor_partial_eq_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let date_spec = Rc::new(DateSpec::new());

    let custom_date1 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor1 = crate::accessors::temporal::date::Date::new(Box::new(custom_date1));
    date_accessor1.set_date(2022, 8, 25).unwrap();

    let custom_date2 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor2 = crate::accessors::temporal::date::Date::new(Box::new(custom_date2));
    date_accessor2.set_date(2022, 8, 25).unwrap();

    let custom_date3 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor3 = crate::accessors::temporal::date::Date::new(Box::new(custom_date3));
    date_accessor3.set_date(2021, 7, 24).unwrap();

    // Test equality
    assert_eq!(date_accessor1, date_accessor2);
    assert_ne!(date_accessor1, date_accessor3);

    // Test hashing
    {
        let mut hasher1 = DefaultHasher::new();
        date_accessor1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        date_accessor2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        date_accessor3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}

#[test]
fn custom_date_adaptor_ord() {
    let date_spec = Rc::new(DateSpec::new());

    let custom_date1 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor1 = crate::accessors::temporal::date::Date::new(Box::new(custom_date1));
    date_accessor1.set_date(2023, 4, 10).unwrap();

    let custom_date2 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor2 = crate::accessors::temporal::date::Date::new(Box::new(custom_date2));
    date_accessor2.set_date(2024, 1, 5).unwrap();

    let custom_date3 = CustomDateAdaptor::new(date_spec.clone());
    let mut date_accessor3 = crate::accessors::temporal::date::Date::new(Box::new(custom_date3));
    date_accessor3.set_date(2023, 4, 10).unwrap();

    // test ordering
    assert!(date_accessor1 < date_accessor2);
    assert!(date_accessor2 > date_accessor1);
    assert!(date_accessor1 <= date_accessor3);
    assert!(date_accessor1 >= date_accessor3);
    assert_eq!(
        date_accessor1.cmp(&date_accessor3),
        std::cmp::Ordering::Equal
    );
}
