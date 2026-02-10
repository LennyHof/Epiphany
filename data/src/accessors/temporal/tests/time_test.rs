use std::rc::Rc;

use crate::{
    accessors::temporal::time::{Time, TimeError},
    adaptor::Adaptor,
    adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
    data_spec_builders::time_spec_builder::TimeSpecBuilder,
    primitive_specs::time_spec::{TimeResolution, TimeSpec, TimeType},
    set_equal_to::SetEqualTo,
    variable,
};

#[test]
fn time_with_no_specified_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 400, 500, 600).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 500);
    assert_eq!(ns, 600);
    assert_eq!(time_accessor.hour().unwrap(), 10);
    assert_eq!(time_accessor.minute().unwrap(), 20);
    assert_eq!(time_accessor.second().unwrap(), 30);
    assert_eq!(time_accessor.millisecond().unwrap(), 400);
    assert_eq!(time_accessor.microsecond().unwrap(), 500);
    assert_eq!(time_accessor.nanosecond().unwrap(), 600);
}

#[test]
fn time_with_nano_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .set_resolution(TimeResolution::Nanosecond)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 400, 500, 600).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 500);
    assert_eq!(ns, 600);
}

#[test]
fn time_with_micro_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .set_resolution(TimeResolution::Microsecond)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 400, 500, 0).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 500);
    assert_eq!(ns, 0);
    let result = time_accessor.set_time(10, 20, 30, 400, 500, 600);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set nanoseconds with value 600 on time with Microsecond resolution."
                .to_string()
        ))
    );
}

#[test]
fn time_with_100_micro_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .set_resolution(TimeResolution::Microsecond100)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 400, 500, 0).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 500);
    assert_eq!(ns, 0);
    let result = time_accessor.set_time(10, 20, 30, 400, 550, 0);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set microseconds with value 550 on time with 100 Microseconds resolution."
                .to_string()
        ))
    );
    let result = time_accessor.set_time(10, 20, 30, 400, 500, 600);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set nanoseconds with value 600 on time with 100 Microseconds resolution."
                .to_string()
        ))
    );
}

#[test]
fn time_with_milli_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .set_resolution(TimeResolution::Millisecond)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 400, 0, 0).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 0);
    assert_eq!(ns, 0);
    let result = time_accessor.set_time(10, 20, 30, 400, 500, 0);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set microseconds with value 500 on time with Millisecond resolution."
                .to_string()
        ))
    );
    let result = time_accessor.set_time(10, 20, 30, 400, 0, 600);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set nanoseconds with value 600 on time with Millisecond resolution."
                .to_string()
        ))
    );
}

#[test]
fn time_with_second_resolution() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .set_resolution(TimeResolution::Second)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor.set_time(10, 20, 30, 0, 0, 0).unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 0);
    assert_eq!(us, 0);
    assert_eq!(ns, 0);
    let result = time_accessor.set_time(10, 20, 30, 400, 0, 0);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set milliseconds with value 400 on time with Second resolution.".to_string()
        ))
    );
    let result = time_accessor.set_time(10, 20, 30, 0, 500, 0);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set microseconds with value 500 on time with Second resolution.".to_string()
        ))
    );
    let result = time_accessor.set_time(10, 20, 30, 0, 0, 600);
    assert!(result.is_err());
    assert_eq!(
        result,
        Err(TimeError::ResolutionOutOfBounds(
            "Cannot set nanoseconds with value 600 on time with Second resolution.".to_string()
        ))
    );
}

#[test]
fn hour_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(24, 0, 0, 0, 0, 0);
    assert_eq!(result, Err(TimeError::HourOutOfBounds(24)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Hour value 24 is out of bounds (0-23)."
    );
}

#[test]
fn minute_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(0, 60, 0, 0, 0, 0);
    assert_eq!(result, Err(TimeError::MinuteOutOfBounds(60)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Minute value 60 is out of bounds (0-59)."
    );
}

#[test]
fn second_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(0, 0, 60, 0, 0, 0);
    assert_eq!(result, Err(TimeError::SecondOutOfBounds(60)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Second value 60 is out of bounds (0-59)."
    );
}

#[test]
fn millisecond_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(0, 0, 0, 1000, 0, 0);
    assert_eq!(result, Err(TimeError::MillisecondOutOfBounds(1000)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Millisecond value 1000 is out of bounds (0-999)."
    );
}

#[test]
fn microsecond_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(0, 0, 0, 0, 1000, 0);
    assert_eq!(result, Err(TimeError::MicrosecondOutOfBounds(1000)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Microsecond value 1000 is out of bounds (0-999)."
    );
}

#[test]
fn nanosecond_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_time(0, 0, 0, 0, 0, 1000);
    assert_eq!(result, Err(TimeError::NanosecondOutOfBounds(1000)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Nanosecond value 1000 is out of bounds (0-999)."
    );
}

#[test]
fn set_time_via_tuple() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor
        .set_via_tuple((10, 20, 30, 400, 500, 600))
        .unwrap();
    let (h, m, s, ms, us, ns) = time_accessor.time().unwrap();
    assert_eq!(h, 10);
    assert_eq!(m, 20);
    assert_eq!(s, 30);
    assert_eq!(ms, 400);
    assert_eq!(us, 500);
    assert_eq!(ns, 600);
}

#[test]
fn set_time_via_tuple_out_of_bounds() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    let result = time_accessor.set_via_tuple((24, 0, 0, 0, 0, 0));
    assert_eq!(result, Err(TimeError::HourOutOfBounds(24)));
    assert_eq!(
        result.unwrap_err().to_string(),
        "Hour value 24 is out of bounds (0-23)."
    );
}

#[test]
fn time_display() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();

    time_accessor
        .set_via_tuple((10, 20, 30, 0, 0, 0))
        .unwrap();
    let display = format!("{}", time_accessor);
    assert_eq!(display, "10:20:30");


    time_accessor
        .set_via_tuple((10, 20, 30, 400, 500, 600))
        .unwrap();
    let display = format!("{}", time_accessor);
    assert_eq!(display, "10:20:30.4005006");
}

#[test]
fn time_debug() {
    let mut var = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor = var.time_mut();
    time_accessor
        .set_via_tuple((10, 20, 30, 400, 500, 600))
        .unwrap();
    let debug = format!("{:?}", time_accessor);
    assert_eq!(
        debug,
        "Time { hour: 10, minute: 20, second: 30, millisecond: 400, microsecond: 500, nanosecond: 600 }"
    );
}

#[test]
fn time_set_equal_to() {
    let mut var1 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let mut var2 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor1 = var1.time_mut();
    let time_accessor2 = var2.time_mut();
    time_accessor1
        .set_via_tuple((10, 20, 30, 400, 500, 600))
        .unwrap();
    time_accessor2.set_via_tuple((0, 0, 0, 0, 0, 0)).unwrap();
    time_accessor2.set_equal_to(time_accessor1).unwrap();
    assert_eq!(time_accessor1, time_accessor2);
    assert_eq!(var1, var2);
}

#[test]
fn time_partial_eq_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut var1 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let mut var2 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let mut var3 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    {
        let time_accessor1 = var1.time_mut();
        let time_accessor2 = var2.time_mut();
        let time_accessor3 = var3.time_mut();
        time_accessor1
            .set_via_tuple((10, 20, 30, 400, 500, 600))
            .unwrap();
        time_accessor2
            .set_via_tuple((10, 20, 30, 400, 500, 600))
            .unwrap();
        time_accessor3
            .set_via_tuple((11, 21, 31, 401, 501, 601))
            .unwrap();
    }

    // test equality and inequality
    assert_eq!(var1, var2);
    assert_eq!(var3, var3);
    assert_ne!(var1, var3);
    assert_eq!(var1.time(), var2.time());
    assert_ne!(var1.time(), var3.time());
    assert_eq!(var3.time(), var3.time());

    // test time hashing
    {
        let mut hasher1 = DefaultHasher::new();
        var1.time().hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        var2.time().hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        var3.time().hash(&mut hasher3);
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
fn time_partial_ord() {
    let mut var1 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let mut var2 = variable::Variable::new(
        &TimeSpecBuilder::new()
            .set_time_type(TimeType::Local)
            .build(),
    );
    let time_accessor1 = var1.time_mut();
    let time_accessor2 = var2.time_mut();
    time_accessor1
        .set_via_tuple((10, 20, 30, 400, 500, 600))
        .unwrap();
    time_accessor2
        .set_via_tuple((11, 21, 31, 401, 501, 601))
        .unwrap();
    assert!(time_accessor1 < time_accessor2);
    assert!(time_accessor2 > time_accessor1);
}

struct CustomTimeAdaptor {
    spec: Rc<TimeSpec>,
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u16,
    microsecond: u16,
    nanosecond: u16,
}

impl CustomTimeAdaptor {
    pub fn new(spec: Rc<TimeSpec>) -> Self {
        Self {
            spec: spec.clone(),
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        }
    }
}

impl Adaptor for CustomTimeAdaptor {}

impl TimeAdaptor for CustomTimeAdaptor {
    fn spec(&self) -> &Rc<TimeSpec> {
        &self.spec
    }

    fn set_time(
        &mut self,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
    ) -> Result<(), TimeError> {
        self.hour = hour;
        self.minute = minute;
        self.second = second;
        self.millisecond = millisecond;
        self.microsecond = microsecond;
        self.nanosecond = nanosecond;
        Ok(())
    }

    fn time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError> {
        Ok((
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            self.microsecond,
            self.nanosecond,
        ))
    }

    fn hour(&self) -> Result<u8, TimeError> {
        Ok(self.hour)
    }

    fn minute(&self) -> Result<u8, TimeError> {
        Ok(self.minute)
    }

    fn second(&self) -> Result<u8, TimeError> {
        Ok(self.second)
    }

    fn millisecond(&self) -> Result<u16, TimeError> {
        Ok(self.millisecond)
    }

    fn microsecond(&self) -> Result<u16, TimeError> {
        Ok(self.microsecond)
    }

    fn nanosecond(&self) -> Result<u16, TimeError> {
        Ok(self.nanosecond)
    }
}

#[test]
fn custom_time_adaptor() {
    let time_spec = Rc::new(TimeSpec::new(Some(TimeType::Local), None));
    let custom_time = CustomTimeAdaptor::new(time_spec.clone());
    let mut time_accessor = Time::new(Box::new(custom_time));

    // Test setting and getting time
    time_accessor.set_time(10, 20, 30, 400, 500, 600).unwrap();
    let time = time_accessor.time().unwrap();
    assert_eq!(time, (10, 20, 30, 400, 500, 600));

    // Test individual components
    assert_eq!(time_accessor.hour().unwrap(), 10);
    assert_eq!(time_accessor.minute().unwrap(), 20);
    assert_eq!(time_accessor.second().unwrap(), 30);
    assert_eq!(time_accessor.millisecond().unwrap(), 400);
    assert_eq!(time_accessor.microsecond().unwrap(), 500);
    assert_eq!(time_accessor.nanosecond().unwrap(), 600);
}

#[test]
fn custom_time_adaptor_partial_ord() {
    let time_spec = Rc::new(TimeSpec::new(Some(TimeType::Local), None));
    let custom_time1 = CustomTimeAdaptor::new(time_spec.clone());
    let custom_time2 = CustomTimeAdaptor::new(time_spec.clone());
    let mut time_accessor1 = Time::new(Box::new(custom_time1));
    let mut time_accessor2 = Time::new(Box::new(custom_time2));

    time_accessor1.set_time(10, 20, 30, 400, 500, 600).unwrap();
    time_accessor2.set_time(11, 21, 31, 401, 501, 601).unwrap();

    assert!(time_accessor1 < time_accessor2);
    assert!(time_accessor2 > time_accessor1);
}

#[test]
fn custom_time_adaptor_ord() {
    let time_spec = Rc::new(TimeSpec::new(Some(TimeType::Local), None));
    let custom_time1 = CustomTimeAdaptor::new(time_spec.clone());
    let custom_time2 = CustomTimeAdaptor::new(time_spec.clone());
    let mut time_accessor1 = Time::new(Box::new(custom_time1));
    let mut time_accessor2 = Time::new(Box::new(custom_time2));

    time_accessor1.set_time(10, 20, 30, 400, 500, 600).unwrap();
    time_accessor2.set_time(11, 21, 31, 401, 501, 601).unwrap();

    assert!(time_accessor1.cmp(&time_accessor2) == std::cmp::Ordering::Less);
    assert!(time_accessor2.cmp(&time_accessor1) == std::cmp::Ordering::Greater);
}

#[test]
fn custom_time_adaptor_partial_equal_and_hash() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let time_spec = Rc::new(TimeSpec::new(Some(TimeType::Local), None));
    let custom_time1 = CustomTimeAdaptor::new(time_spec.clone());
    let custom_time2 = CustomTimeAdaptor::new(time_spec.clone());
    let custom_time3 = CustomTimeAdaptor::new(time_spec.clone());

    let mut time_accessor1 = Time::new(Box::new(custom_time1));
    let mut time_accessor2 = Time::new(Box::new(custom_time2));
    let mut time_accessor3 = Time::new(Box::new(custom_time3));

    time_accessor1.set_time(10, 20, 30, 400, 500, 600).unwrap();
    time_accessor2.set_time(10, 20, 30, 400, 500, 600).unwrap();
    time_accessor3.set_time(11, 21, 31, 401, 501, 601).unwrap();

    assert_eq!(time_accessor1, time_accessor2);
    assert_ne!(time_accessor1, time_accessor3);

    let mut hasher1 = DefaultHasher::new();
    time_accessor1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    time_accessor2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    let mut hasher3 = DefaultHasher::new();
    time_accessor3.hash(&mut hasher3);
    let hash3 = hasher3.finish();

    assert_eq!(hash1, hash2);
    assert_ne!(hash1, hash3);
}
