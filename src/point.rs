use ndarray::Array1;
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
            values: Array1::from_shape_vec(values.len(), values).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0., 0., vec![1., 2., 3.])]
    #[case(90.1234567, -54.7654321, vec![f64::MAX; 9999])]
    fn test_new(#[case] x: f64, #[case] y: f64, #[case] values: Vec<f64>) {
        let point = Point::new(x, y, values.clone());
        assert_eq!(point.x, x);
        assert_eq!(point.y, y);
        assert_eq!(point.values, Array1::from_vec(values));
    }
}
