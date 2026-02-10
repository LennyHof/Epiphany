use std::rc::Rc;

use crate::{
    accessors::temporal::time::TimeError, adaptor::Adaptor,
    adaptors::temporal_adaptors::time_adaptor::TimeAdaptor,
    adaptors::temporal_adaptors::time_adaptor_100_microseconds::TimeAdaptor100Microseconds,
    primitive_specs::time_spec::TimeSpec,
};

pub struct TransientTimeMicroAdaptor {
    spec: Rc<TimeSpec>,
    ticks: u32,
}

impl TransientTimeMicroAdaptor {
    pub fn new(spec: Rc<TimeSpec>) -> Self {
        Self { spec, ticks: 0 }
    }
}

impl Adaptor for TransientTimeMicroAdaptor {}

impl TimeAdaptor for TransientTimeMicroAdaptor {
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
        _nanosecond: u16,
    ) -> Result<(), TimeError> {
        self.do_set_time(hour, minute, second, millisecond, microsecond)
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
        Ok(0)
    }
}

impl TimeAdaptor100Microseconds for TransientTimeMicroAdaptor {
    fn set_ticks(&mut self, ticks: u32) -> Result<(), TimeError> {
        self.ticks = ticks;
        Ok(())
    }

    fn ticks(&self) -> Result<u32, TimeError> {
        Ok(self.ticks)
    }
}
