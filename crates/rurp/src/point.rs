use ndarray::Array1;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub values: Array1<f64>,
}

impl Point {
    pub fn new(x: f64, y: f64, values: Vec<f64>) -> Self {
        Point {
            x,
            y,
            values: Array1::from_vec(values),
        }
    }
}
