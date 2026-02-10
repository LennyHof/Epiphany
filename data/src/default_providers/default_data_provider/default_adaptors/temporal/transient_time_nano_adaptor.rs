use std::rc::Rc;

use crate::{
    accessors::temporal::time::TimeError, adaptor::Adaptor,
    adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
    adaptors::temporal_adaptors::time_adaptor_nanosecond::TimeAdaptorNanosecond,
    primitive_specs::time_spec::TimeSpec,
};

pub struct TransientTimeNanoAdaptor {
    spec: Rc<TimeSpec>,
    nanoseconds: u64,
}
impl TransientTimeNanoAdaptor {
    pub fn new(spec: Rc<TimeSpec>) -> Self {
        Self {
            spec,
            nanoseconds: 0,
        }
    }
}
impl Adaptor for TransientTimeNanoAdaptor {}

impl TimeAdaptor for TransientTimeNanoAdaptor {
    fn spec(&self) -> &Rc<TimeSpec> {
        &self.spec
    }

    fn can_return_time_as_nanos(&self) -> bool {
        true
    }

    fn nanos(&self) -> Result<u64, TimeError> {
        Ok(self.nanoseconds)
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
        self.do_set_time(hour, minute, second, millisecond, microsecond, nanosecond)
    }

    fn time(&self) -> Result<(u8, u8, u8, u16, u16, u16), TimeError> {
        self.do_get_time()
    }

    fn hour(&self) -> Result<u8, TimeError> {
        self.do_get_hour()
    }

    fn minute(&self) -> Result<u8, TimeError> {
        self.do_get_minute()
    }

    fn second(&self) -> Result<u8, TimeError> {
        self.do_get_second()
    }

    fn millisecond(&self) -> Result<u16, TimeError> {
        self.do_get_millisecond()
    }

    fn microsecond(&self) -> Result<u16, TimeError> {
        self.do_get_microsecond()
    }

    fn nanosecond(&self) -> Result<u16, TimeError> {
        self.do_get_nanosecond()
    }
}

impl TimeAdaptorNanosecond for TransientTimeNanoAdaptor {
    fn set_nanoseconds(&mut self, nanoseconds: u64) -> Result<(), TimeError> {
        self.nanoseconds = nanoseconds;
        Ok(())
    }

    fn nanoseconds(&self) -> Result<u64, TimeError> {
        Ok(self.nanoseconds)
    }
}
