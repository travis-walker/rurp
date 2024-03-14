#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub values: ndarray::Array1<f64>,
}

impl Point {
    /// Create a new Point instance.
    #[must_use]
    pub fn new(x: f64, y: f64, values: Vec<f64>) -> Self {
        Point {
            x,
            y,
            values: ndarray::Array1::from_vec(values),
        }
    }
}

impl spade::HasPosition for Point {
    type Scalar = f64;
    fn position(&self) -> spade::Point2<Self::Scalar> {
        spade::Point2::new(self.x, self.y)
    }
}

/// Convert a Point ref into a tuple of f64s.
impl From<&Point> for (f64, f64) {
    fn from(value: &Point) -> Self {
        (value.x, value.y)
    }
}

/// Convert a Point ref into an array of f64s.
impl From<&Point> for [f64; 2] {
    fn from(value: &Point) -> Self {
        [value.x, value.y]
    }
}

/// Convert a Point ref into a geo Point.
impl From<&Point> for geo::Point {
    fn from(value: &Point) -> Self {
        geo::Point::new(value.x, value.y)
    }
}

/// Convert a Point ref into a geo Coord.
impl From<&Point> for geo::Coord {
    fn from(value: &Point) -> Self {
        geo::Coord {
            x: value.x,
            y: value.y,
        }
    }
}

/// Convert a Point ref into a delaunator Point.
impl From<&Point> for voronator::delaunator::Point {
    fn from(value: &Point) -> Self {
        voronator::delaunator::Point {
            x: value.x,
            y: value.y,
        }
    }
}
