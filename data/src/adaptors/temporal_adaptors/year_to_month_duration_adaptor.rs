use std::rc::Rc;

use crate::{
    accessors::temporal::year_to_month_duration::YearToMonthDurationError, adaptor::Adaptor,
    primitive_specs::duration_spec::DurationSpec,
};

/// An adaptor for year-month durations.
pub trait YearToMonthDurationAdaptor: Adaptor {
    /// Returns the duration's specification.
    fn spec(&self) -> &Rc<DurationSpec>;

    /// Returns true if the duration is stored as total months, false otherwise.
    fn stores_duration_as_months(&self) -> bool;

    /// Sets the duration in total months.
    fn set_total_months(&mut self, _months: i32) -> Result<(), YearToMonthDurationError> {
        unimplemented!()
    }

    /// Returns the duration in total months.
    fn total_months(&self) -> Result<i32, YearToMonthDurationError> {
        unimplemented!()
    }

    /// Sets the duration in years and months.
    fn set_duration(&mut self, years: i32, months: i32) -> Result<(), YearToMonthDurationError> {
        if self.stores_duration_as_months() {
            self.set_total_months(years * 12 + months)
        } else {
            unimplemented!()
        }
    }

    /// Returns the duration as a tuple of (years, months).
    fn duration(&self) -> Result<(i32, i32), YearToMonthDurationError> {
        if self.stores_duration_as_months() {
            let months = self.total_months()?;
            Ok((months / 12, months % 12))
        } else {
            unimplemented!()
        }
    }

    /// Returns the number of years in the duration.
    fn years(&self) -> Result<i32, YearToMonthDurationError> {
        if self.stores_duration_as_months() {
            let months = self.total_months()?;
            return Ok(months / 12);
        }
        let (years, _) = self.duration()?;
        Ok(years)
    }

    /// Returns the number of months in the duration.
    fn months(&self) -> Result<i32, YearToMonthDurationError> {
        if self.stores_duration_as_months() {
            let months = self.total_months()?;
            return Ok(months % 12);
        }
        let (_, months) = self.duration()?;
        Ok(months)
    }
}
