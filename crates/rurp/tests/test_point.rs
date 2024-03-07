use ndarray::Array1;
use rstest::rstest;
use rurp::point::Point;

#[rstest]
#[case(0., 0., vec![1., 2., 3.])]
#[case(90.123_456_7, -54.765_432_1, vec![f64::MAX; 9999])]
fn test_new(#[case] x: f64, #[case] y: f64, #[case] values: Vec<f64>) {
    let point = Point::new(x, y, values.clone());
    assert_eq!(point.x, x);
    assert_eq!(point.y, y);
    assert_eq!(point.values, Array1::from_vec(values));
}

#[test]
fn test_into_tuple() {
    let point = Point::new(100., -20., vec![1.]);
    let tup: (f64, f64) = (&point).into();
    assert_eq!(tup.0, point.x);
    assert_eq!(tup.1, point.y);
}
#[test]
fn test_into_array() {
    let point = Point::new(100., -20., vec![1.]);
    let arr: [f64; 2] = (&point).into();
    assert_eq!(arr[0], point.x);
    assert_eq!(arr[1], point.y);
}
#[test]
fn test_into_geo_point() {
    let point = Point::new(100., -20., vec![1.]);
    let geo_point: geo::Point = (&point).into();
    assert_eq!(geo_point.x(), point.x);
    assert_eq!(geo_point.y(), point.y);
}
#[test]
fn test_into_geo_coord() {
    let point = Point::new(100., -20., vec![1.]);
    let geo_coord: geo::Coord = (&point).into();
    assert_eq!(geo_coord.x, point.x);
    assert_eq!(geo_coord.y, point.y);
}
#[test]
fn test_into_delaunator_point() {
    let point = Point::new(100., -20., vec![1.]);
    let delaunator_point: voronator::delaunator::Point = (&point).into();
    assert_eq!(delaunator_point.x, point.x);
    assert_eq!(delaunator_point.y, point.y);
}
