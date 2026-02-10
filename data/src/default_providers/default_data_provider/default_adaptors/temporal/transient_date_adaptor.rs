use std::rc::Rc;

use crate::accessors::temporal::date::DateError;
use crate::adaptor::Adaptor;
use crate::adaptors::temporal_adaptors::date_adaptor::DateAdaptor;
use crate::primitive_specs::date_spec::DateSpec;

/// A transient date adaptor that stores the date as the number of days in memory.
pub struct TransientDateAdaptor {
    spec: Rc<DateSpec>,
    days: u32,
}

impl TransientDateAdaptor {
    /// Creates a new transient date adaptor.
    pub fn new(spec: Rc<DateSpec>) -> Self {
        Self {
            spec: spec.clone(),
            days: 0,
        }
    }
}

impl Adaptor for TransientDateAdaptor {}

impl DateAdaptor for TransientDateAdaptor {
    fn spec(&self) -> &Rc<DateSpec> {
        &self.spec
    }

    fn stores_date_as_days(&self) -> bool {
        true
    }

    fn days(&self) -> Result<u32, DateError> {
        Ok(self.days)
    }

    fn set_days(&mut self, days: u32) -> Result<(), DateError> {
        self.days = days;
        Ok(())
    }
}
