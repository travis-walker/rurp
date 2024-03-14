use std::error::Error;
#[derive(Debug, Clone, PartialEq)]
pub struct Bounds {
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
}

impl Bounds {
    /// Get the left bound.
    #[must_use]
    pub fn left(&self) -> f64 {
        self.left
    }
    /// Get the bottom bound.
    #[must_use]
    pub fn bottom(&self) -> f64 {
        self.bottom
    }
    /// Get the right bound.
    #[must_use]
    pub fn right(&self) -> f64 {
        self.right
    }
    /// Get the top bound.
    #[must_use]
    pub fn top(&self) -> f64 {
        self.top
    }
}

impl Bounds {
    /// Create a new Bounds instance.
    /// # Errors
    /// Returns an error if the left bound is greater than or equal to the right bound or if the bottom bound is greater than or equal to the top bound.
    pub fn new(left: f64, bottom: f64, right: f64, top: f64) -> Result<Self, Box<dyn Error>> {
        if left >= right {
            return Err("left must be less than right".into());
        }
        if bottom >= top {
            return Err("bottom must be less than top".into());
        }
        Ok(Self {
            left,
            bottom,
            right,
            top,
        })
    }
}

impl TryFrom<(f64, f64, f64, f64)> for Bounds {
    type Error = Box<dyn Error>;
    /// Convert a tuple of f64s into a Bounds instance.
    /// # Errors
    /// Returns an error if the left bound is greater than or equal to the right bound or if the bottom bound is greater than or equal to the top bound.
    fn try_from(value: (f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        Bounds::new(value.0, value.1, value.2, value.3)
    }
}

impl From<Bounds> for (f64, f64, f64, f64) {
    /// Convert a Bounds instance into a tuple of f64s.
    fn from(bounds: Bounds) -> (f64, f64, f64, f64) {
        (bounds.left, bounds.bottom, bounds.right, bounds.top)
    }
}

impl TryFrom<[f64; 4]> for Bounds {
    type Error = Box<dyn Error>;
    /// Convert an array of f64s into a Bounds instance.
    /// # Errors
    /// Returns an error if the left bound is greater than or equal to the right bound or if the bottom bound is greater than or equal to the top bound.
    fn try_from(value: [f64; 4]) -> Result<Self, Self::Error> {
        Bounds::new(value[0], value[1], value[2], value[3])
    }
}
impl From<Bounds> for [f64; 4] {
    /// Convert a Bounds instance into an array of f64s.
    fn from(bounds: Bounds) -> [f64; 4] {
        [bounds.left, bounds.bottom, bounds.right, bounds.top]
    }
}
