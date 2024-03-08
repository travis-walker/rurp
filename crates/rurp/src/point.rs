use ndarray::Array1;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub values: Array1<f64>,
}

impl Point {
    /// Create a new Point instance.
    #[must_use]
    pub fn new(x: f64, y: f64, values: Vec<f64>) -> Self {
        Point {
            x,
            y,
            values: Array1::from_vec(values),
        }
    }
}

/// Convert a Point instance into a tuple of f64s.
impl From<&Point> for (f64, f64) {
    fn from(value: &Point) -> Self {
        (value.x, value.y)
    }
}

/// Convert a Point instance into an array of f64s.
impl From<&Point> for [f64; 2] {
    fn from(value: &Point) -> Self {
        [value.x, value.y]
    }
}

/// Convert a Point instance into a geo Point.
impl From<&Point> for geo::Point {
    fn from(value: &Point) -> Self {
        geo::Point::new(value.x, value.y)
    }
}

/// Convert a Point instance into a geo Coord.
impl From<&Point> for geo::Coord {
    fn from(value: &Point) -> Self {
        geo::Coord {
            x: value.x,
            y: value.y,
        }
    }
}

/// Convert a Point instance into a delaunator Point.
impl From<&Point> for voronator::delaunator::Point {
    fn from(value: &Point) -> Self {
        voronator::delaunator::Point {
            x: value.x,
            y: value.y,
        }
    }
}
