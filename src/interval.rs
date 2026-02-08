use std::ops::RangeInclusive;

pub trait Interval {
    fn surrounds(&self, value: &f64) -> bool;
}

// ---------------------------------------------------------------------------

impl Interval for RangeInclusive<f64> {
    fn surrounds(&self, value: &f64) -> bool {
        self.start() < value && value < self.end()
    }
}
