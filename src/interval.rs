#[derive(Copy, Clone)]
pub struct Interval {
    pub(crate) min: f64,
    pub(crate) max: f64,
}

impl Interval {
    pub const fn new() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
        }
    }

    pub const fn from(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn casting_default() -> Self{
        Self{min: 0.000001, max: f64::MAX}
    }

    pub const fn from_intervals(a: &Self, b: &Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new()
    }
}
