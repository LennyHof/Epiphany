use std::rc::Rc;

use crate::{
    adaptor::Adaptor,
    adaptors::temporal_adaptors::year_to_month_duration_adaptor::YearToMonthDurationAdaptor,
    primitive_specs::duration_spec::DurationSpec,
};

pub struct TransientYearToMonthDurationAdaptor {
    spec: Rc<DurationSpec>,
    total_months: i32,
}

impl TransientYearToMonthDurationAdaptor {
    // creates a new TransientYearToMonthDurationAdaptor with the given DurationSpec
    pub fn new(spec: Rc<DurationSpec>) -> Self {
        Self {
            spec,
            total_months: 0,
        }
    }
}

impl Adaptor for TransientYearToMonthDurationAdaptor {}

impl YearToMonthDurationAdaptor for TransientYearToMonthDurationAdaptor {
    fn spec(&self) -> &Rc<DurationSpec> {
        &self.spec
    }

    fn stores_duration_as_months(&self) -> bool {
        true
    }

    fn set_total_months(
        &mut self,
        months: i32,
    ) -> Result<(), crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError>
    {
        self.total_months = months;
        Ok(())
    }

    fn total_months(
        &self,
    ) -> Result<i32, crate::accessors::temporal::year_to_month_duration::YearToMonthDurationError>
    {
        Ok(self.total_months)
    }
}
