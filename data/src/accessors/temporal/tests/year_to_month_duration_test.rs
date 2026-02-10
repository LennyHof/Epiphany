use std::rc::Rc;

use crate::{
    accessors::temporal::year_to_month_duration::{YearToMonthDuration, YearToMonthDurationError},
    adaptor::Adaptor,
    adaptors::temporal_adaptors::year_to_month_duration_adaptor::YearToMonthDurationAdaptor,
    data_spec_builders::duration_spec_builder::DurationSpecBuilder,
    primitive_specs::duration_spec::{DurationResolution, DurationSpec, DurationType},
    set_equal_to::SetEqualTo,
    variable::Variable,
};

#[test]
fn default_year_to_month_duration() {
    let var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let ymd = var.year_to_month_duration();
    assert_eq!(ymd.duration().unwrap(), (0, 0));
    assert_eq!(ymd.years().unwrap(), 0);
    assert_eq!(ymd.months().unwrap(), 0);
}

#[test]
fn positive_year_to_month_duration() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(1, 2).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (1, 2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 2);
}

#[test]
fn negative_year_to_month_duration() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(-1, -2).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (-1, -2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), -1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), -2);
}

#[test]
fn pos_years_neg_months_duration() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(1, -2).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (0, 10));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 0);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 10);
}

#[test]
fn neg_years_pos_months_duration() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(-2, 4).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (-1, -8));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), -1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), -8);
}

#[test]
fn overflow_months_no_years() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(0, 14).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (1, 2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 2);
}

#[test]
fn overflow_months_with_years() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(1, 14).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (2, 2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 2);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 2);
}

#[test]
fn resolution_out_of_bounds_error() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .set_resolution(DurationResolution::Year)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    let result = year_to_month_duration_mut.set_duration(1, 1);
    assert_eq!(
        result,
        Err(YearToMonthDurationError::ResolutionOutOfBounds(
            "Cannot set months with value 1 on a year-to-month-duration with a Year resolution."
                .to_string()
        ))
    );
}

#[test]
fn ymd_display() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(1, 2).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (1, 2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 2);
    assert_eq!(format!("{}", year_to_month_duration_mut), "P1Y2M");
}

#[test]
fn ymd_debug() {
    let mut var = Variable::new(
        &DurationSpecBuilder::new()
            .set_type(DurationType::YearToMonth)
            .build()
            .unwrap(),
    );
    let year_to_month_duration_mut = var.year_to_month_duration_mut();
    year_to_month_duration_mut.set_duration(1, 2).unwrap();
    assert_eq!(year_to_month_duration_mut.duration().unwrap(), (1, 2));
    assert_eq!(year_to_month_duration_mut.years().unwrap(), 1);
    assert_eq!(year_to_month_duration_mut.months().unwrap(), 2);
    assert_eq!(
        format!("{:?}", year_to_month_duration_mut),
        "YearToMonthDuration { years: 1, months: 2 }"
    );
}

#[test]
fn ymd_set_equal_to() {
    let spec = DurationSpecBuilder::new()
        .set_type(DurationType::YearToMonth)
        .build()
        .unwrap();

    // set value to be copied
    let mut var1_mut = Variable::new(&spec);
    let ymd1_mut = var1_mut.year_to_month_duration_mut();
    ymd1_mut.set_duration(12, 3).unwrap();

    // test set equal to via accessor
    let mut var2_mut = Variable::new(&spec);
    let ymd2_mut = var2_mut.year_to_month_duration_mut();
    ymd2_mut.set_equal_to(&ymd1_mut).unwrap();
    assert_eq!(ymd2_mut.duration().unwrap(), (12, 3));

    // test set equal to via variable
    let mut var3_mut = Variable::new(&spec);
    var3_mut.set_equal_to(&var1_mut).unwrap();
    assert_eq!(
        var3_mut.year_to_month_duration().duration().unwrap(),
        (12, 3)
    );
}

#[test]
fn ymd_partial_eq_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let spec = DurationSpecBuilder::new()
        .set_type(DurationType::YearToMonth)
        .build()
        .unwrap();

    let mut var1_mut = Variable::new(&spec);
    let mut var2_mut = Variable::new(&spec);
    let mut var3_mut = Variable::new(&spec);

    {
        let ymd1_mut = var1_mut.year_to_month_duration_mut();
        ymd1_mut.set_duration(10, 9).unwrap();
        let ymd2_mut = var2_mut.year_to_month_duration_mut();
        ymd2_mut.set_duration(10, 9).unwrap();
        let ymd3_mut = var3_mut.year_to_month_duration_mut();
        ymd3_mut.set_duration(9, 8).unwrap();
    }

    // test accessor equality
    assert_eq!(
        var1_mut.year_to_month_duration(),
        var2_mut.year_to_month_duration()
    );
    assert_ne!(
        var1_mut.year_to_month_duration(),
        var3_mut.year_to_month_duration()
    );

    // test variable equality
    assert_eq!(var1_mut, var2_mut);
    assert_ne!(var1_mut, var3_mut);

    // test accessor hashing
    {
        let mut hasher1 = DefaultHasher::new();
        var1_mut.year_to_month_duration().hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        var2_mut.year_to_month_duration().hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        var3_mut.year_to_month_duration().hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    // test variable hashing
    {
        let mut hasher1 = DefaultHasher::new();
        var1_mut.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        var2_mut.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        var3_mut.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }
}

#[test]
fn ymd_ord() {
    let spec = DurationSpecBuilder::new()
        .set_type(DurationType::YearToMonth)
        .build()
        .unwrap();

    let mut var1_mut = Variable::new(&spec);
    let mut var2_mut = Variable::new(&spec);
    let mut var3_mut = Variable::new(&spec);

    {
        let ymd1_mut = var1_mut.year_to_month_duration_mut();
        ymd1_mut.set_duration(10, 9).unwrap();
        let ymd2_mut = var2_mut.year_to_month_duration_mut();
        ymd2_mut.set_duration(10, 10).unwrap();
        let ymd3_mut = var3_mut.year_to_month_duration_mut();
        ymd3_mut.set_duration(10, 9).unwrap();
    }

    // test accessor ordering
    let ymd1 = var1_mut.year_to_month_duration();
    let ymd2 = var2_mut.year_to_month_duration();
    let ymd3 = var3_mut.year_to_month_duration();
    assert!(ymd1 < ymd2);
    assert!(ymd2 > ymd1);
    assert!(ymd1 <= ymd3);
    assert!(ymd1 >= ymd3);

    // test variable ordering
    assert!(var1_mut < var2_mut);
    assert!(var2_mut > var1_mut);
    assert!(var1_mut <= var3_mut);
    assert!(var1_mut >= var3_mut);
}

struct CustomYmdAdaptor {
    spec: Rc<DurationSpec>,
    years: i32,
    months: i32,
}

impl CustomYmdAdaptor {
    pub fn new(spec: Rc<DurationSpec>) -> Self {
        Self {
            spec,
            years: 0,
            months: 0,
        }
    }
}

impl Adaptor for CustomYmdAdaptor {}

impl YearToMonthDurationAdaptor for CustomYmdAdaptor {
    fn spec(&self) -> &Rc<DurationSpec> {
        &self.spec
    }

    fn stores_duration_as_months(&self) -> bool {
        false
    }

    fn set_duration(
        &mut self,
        years: i32,
        months: i32,
    ) -> Result<(), crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError>
    {
        self.years = years;
        self.months = months;
        Ok(())
    }

    fn duration(
        &self,
    ) -> Result<
        (i32, i32),
        crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError,
    > {
        Ok((self.years, self.months))
    }

    fn years(
        &self,
    ) -> Result<i32, crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError>
    {
        Ok(self.years)
    }

    fn months(
        &self,
    ) -> Result<i32, crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError>
    {
        Ok(self.months)
    }
}

#[test]
fn custom_ymd_adaptor() {
    let spec = Rc::new(DurationSpec::new(Some(DurationType::YearToMonth), None));
    let custom_adaptor = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor = YearToMonthDuration::new(Box::new(custom_adaptor));
    custom_accessor.set_duration(5, 8).unwrap();
    assert_eq!(custom_accessor.duration().unwrap(), (5, 8));
}

#[test]
fn custom_ymd_adaptor_set_equal_to() {
    let spec = Rc::new(DurationSpec::new(Some(DurationType::YearToMonth), None));

    let custom_adaptor1 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor1 = YearToMonthDuration::new(Box::new(custom_adaptor1));
    custom_accessor1.set_duration(9, 7).unwrap();

    let custom_adaptor2 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor2 = YearToMonthDuration::new(Box::new(custom_adaptor2));
    custom_accessor2.set_equal_to(&custom_accessor1).unwrap();

    assert_eq!(custom_accessor2.duration().unwrap(), (9, 7));
}

#[test]
fn custom_ymd_adaptor_partial_eq_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let spec = Rc::new(DurationSpec::new(Some(DurationType::YearToMonth), None));

    let custom_adaptor1 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor1 = YearToMonthDuration::new(Box::new(custom_adaptor1));
    custom_accessor1.set_duration(9, 7).unwrap();

    let custom_adaptor2 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor2 = YearToMonthDuration::new(Box::new(custom_adaptor2));
    custom_accessor2.set_duration(9, 7).unwrap();

    let custom_adaptor3 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor3 = YearToMonthDuration::new(Box::new(custom_adaptor3));
    custom_accessor3.set_duration(8, 6).unwrap();

    // test equality
    assert_eq!(custom_accessor1, custom_accessor2);
    assert_ne!(custom_accessor1, custom_accessor3);

    // test hashing
    let mut hasher1 = DefaultHasher::new();
    custom_accessor1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    custom_accessor2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    let mut hasher3 = DefaultHasher::new();
    custom_accessor3.hash(&mut hasher3);
    let hash3 = hasher3.finish();

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
}

#[test]
fn custom_ymd_adaptor_ord() {
    let spec = Rc::new(DurationSpec::new(Some(DurationType::YearToMonth), None));

    let custom_adaptor1 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor1 = YearToMonthDuration::new(Box::new(custom_adaptor1));
    custom_accessor1.set_duration(9, 7).unwrap();

    let custom_adaptor2 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor2 = YearToMonthDuration::new(Box::new(custom_adaptor2));
    custom_accessor2.set_duration(11, 10).unwrap();

    let custom_adaptor3 = CustomYmdAdaptor::new(spec.clone());
    let mut custom_accessor3 = YearToMonthDuration::new(Box::new(custom_adaptor3));
    custom_accessor3.set_duration(9, 7).unwrap();

    // test ordering
    assert!(custom_accessor1 < custom_accessor2);
    assert!(custom_accessor2 > custom_accessor3);
    assert!(custom_accessor1 <= custom_accessor3);
    assert!(custom_accessor3 >= custom_accessor1);
    assert_eq!(
        custom_accessor1.cmp(&custom_accessor3),
        std::cmp::Ordering::Equal
    );
}
