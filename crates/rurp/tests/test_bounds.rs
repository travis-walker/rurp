use rstest::rstest;
use rurp::bounds::Bounds;

#[rstest]
#[case(0., 1., 2., 3.)]
#[case(-20_000., -30_000., 2_500., 600_000.)]
fn test_new(#[case] left: f64, #[case] bottom: f64, #[case] right: f64, #[case] top: f64) {
    let bounds = Bounds::new(left, bottom, right, top).unwrap();
    assert_eq!(bounds.left(), left);
    assert_eq!(bounds.bottom(), bottom);
    assert_eq!(bounds.right(), right);
    assert_eq!(bounds.top(), top);
}

#[rstest]
#[case::gt(1.0, 0.0)]
#[case::eq(0.0, 0.0)]
fn test_error_if_left_gte_right(#[case] left: f64, #[case] right: f64) {
    let result = Bounds::new(left, 1.0, right, 3.0);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("left must be less than right"));
}

#[rstest]
#[case::gt(1.0, 0.0)]
#[case::eq(0.0, 0.0)]
fn test_error_if_bottom_gte_top(#[case] bottom: f64, #[case] top: f64) {
    let result = Bounds::new(0.0, bottom, 1.0, top);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("bottom must be less than top"));
}

#[rstest]
fn test_from_tuple() {
    let bounds_tup = (-2_221_060., 523_589., 3_181_702., 3_363_319.);
    let bounds: Bounds = bounds_tup.try_into().unwrap();
    assert_eq!(bounds.left(), bounds_tup.0);
    assert_eq!(bounds.bottom(), bounds_tup.1);
    assert_eq!(bounds.right(), bounds_tup.2);
    assert_eq!(bounds.top(), bounds_tup.3);
}

#[rstest]
fn test_from_array() {
    let bounds_tup = [-2., -2., 0., 0.];
    let bounds: Bounds = bounds_tup.try_into().unwrap();
    assert_eq!(bounds.left(), bounds_tup[0]);
    assert_eq!(bounds.bottom(), bounds_tup[1]);
    assert_eq!(bounds.right(), bounds_tup[2]);
    assert_eq!(bounds.top(), bounds_tup[3]);
}
