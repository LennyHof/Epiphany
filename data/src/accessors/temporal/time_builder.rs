/// An utility to build time components for setting via a `Time` accessor.
pub struct TimeBuilder {
    hour: u8,
    minute: u8,
    second: u8,
    millisecond: u32,
    microsecond: u32,
    nanosecond: u32,
}

impl TimeBuilder {
    /// Creates a new `TimeBuilder` instance.
    pub fn new() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
            microsecond: 0,
            nanosecond: 0,
        }
    }

    /// Sets the hour component.
    pub fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }

    /// Sets the minute component.
    pub fn minute(mut self, minute: u8) -> Self {
        self.minute = minute;
        self
    }

    /// Sets the second component.
    pub fn second(mut self, second: u8) -> Self {
        self.second = second;
        self
    }

    /// Sets the millisecond component.
    pub fn millisecond(mut self, millisecond: u32) -> Self {
        self.millisecond = millisecond;
        self
    }

    /// Sets the microsecond component.
    pub fn microsecond(mut self, microsecond: u32) -> Self {
        self.microsecond = microsecond;
        self
    }

    /// Sets the nanosecond component.
    pub fn nanosecond(mut self, nanosecond: u32) -> Self {
        self.nanosecond = nanosecond;
        self
    }

    /// Builds the time components into a tuple.
    /// Returns a tuple of (hour, minute, second, millisecond, microsecond, nanosecond).
    pub fn build(self) -> (u8, u8, u8, u32, u32, u32) {
        (
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            self.microsecond,
            self.nanosecond,
        )
    }
}
