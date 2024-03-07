use ndarray::Array1;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub values: Array1<f64>,
}

impl Point {
    #[must_use] pub fn new(x: f64, y: f64, values: Vec<f64>) -> Self {
        Point {
            x,
            y,
            values: Array1::from_vec(values),
        }
    }
}

impl From<&Point> for (f64, f64) {
    fn from(value: &Point) -> Self {
        (value.x, value.y)
    }
}
impl From<&Point> for [f64; 2] {
    fn from(value: &Point) -> Self {
        [value.x, value.y]
    }
}
impl From<&Point> for geo::Point {
    fn from(value: &Point) -> Self {
        geo::Point::new(value.x, value.y)
    }
}
impl From<&Point> for geo::Coord {
    fn from(value: &Point) -> Self {
        geo::Coord {
            x: value.x,
            y: value.y,
        }
    }
}
impl From<&Point> for voronator::delaunator::Point {
    fn from(value: &Point) -> Self {
        voronator::delaunator::Point {
            x: value.x,
            y: value.y,
        }
    }
}
