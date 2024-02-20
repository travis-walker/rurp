use ndarray::{Array1, Array2, Array3};

#[derive(Debug)]
pub struct Grid {
    pub nodata: f64,
    pub data: Array3<f64>,
    pub x: Array2<f64>,
    pub y: Array2<f64>,
}

impl Grid {
    pub fn empty_from_bounds(
        nodata: f64,
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        resolution: usize,
    ) -> Self {
        let width = ((right - left) / resolution as f64).ceil() as usize;
        let height = ((top - bottom) / resolution as f64).ceil() as usize;

        let data = Array3::from_elem((height, width, 1), nodata);

        let x_vals = Array1::linspace(left, right, width);
        let x = Array2::from_shape_fn((height, width), |(_, j)| x_vals[j]);

        let y_vals = Array1::linspace(bottom, top, height);
        let y = Array2::from_shape_fn((height, width), |(i, _)| y_vals[i]);

        Grid { nodata, data, x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn equivalent(left: f64, right: f64) -> bool {
        left.is_nan() && right.is_nan() || left == right
    }

    #[rstest]
    #[case(4000, 710, 1351, f64::NAN)]
    #[case(8000, 355, 676, f64::NEG_INFINITY)]
    #[case(1200, 2367, 4503, f64::MAX)]
    fn test_from_bounds(
        #[case] resolution: usize,
        #[case] expected_height: usize,
        #[case] expected_width: usize,
        #[case] nodata: f64,
    ) {
        let left = -2221060.;
        let bottom = 523589.;
        let right = 3181702.;
        let top = 3363319.;

        let grid = Grid::empty_from_bounds(nodata, left, bottom, right, top, resolution);

        assert_eq!(grid.data.shape(), [expected_height, expected_width, 1]);

        assert!(equivalent(grid.data[[0, 0, 0]], nodata));
        assert_eq!(grid.x[[0, 0]], left);
        assert_eq!(grid.y[[0, 0]], bottom);

        let max_y = expected_height - 1;
        let max_x = expected_width - 1;
        assert!(equivalent(grid.data[[max_y, max_x, 0]], nodata));
        assert_eq!(grid.x[[max_y, max_x]], right);
        assert_eq!(grid.y[[max_y, max_x]], top);

        let mid_y = expected_height / 2;
        let mid_x = expected_width / 2;
        assert!(equivalent(grid.data[[mid_y, mid_x, 0]], nodata));
        assert!((grid.x[[mid_y, mid_x]] - ((right + left) / 2.)).abs() < resolution as f64);
        assert!((grid.y[[mid_y, mid_x]] - ((top + bottom) / 2.)).abs() < resolution as f64);
    }
}
