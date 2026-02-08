use std::ops::RangeInclusive;

pub trait Interval {
    fn surrounds(&self, value: &f64) -> bool;
    fn clamp(&self, value: &f64) -> f64;
}

// ---------------------------------------------------------------------------

impl Interval for RangeInclusive<f64> {
    fn surrounds(&self, value: &f64) -> bool {
        self.start() < value && value < self.end()
    }

    // -----------------------------------------------------------------------

    fn clamp(&self, value: &f64) -> f64 {
        if value < self.start() {
            *self.start()
        } else if self.end() < value {
            *self.end()
        } else {
            *value
        }
    }
}
