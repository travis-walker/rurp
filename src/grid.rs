use ndarray::{Array1, Array2, Array3};

pub struct Grid {
    pub nodata: f64,
    pub data: Array3<f64>,
    pub x: Array2<f64>,
    pub y: Array2<f64>,
}

impl Grid {
    pub fn empty_from_bounds(
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        resolution: usize,
        nodata: f64,
    ) -> Self {
        let width = (right - left) as usize / resolution;
        let height = (top - bottom) as usize / resolution;
        let data = Array3::from_elem((height, width, 1), nodata);
        let x_vals = Array1::linspace(left, right, width);
        let y_vals = Array1::linspace(bottom, top, height);
        let x = Array2::from_shape_fn((height, width), |(_, j)| x_vals[j]);
        let y = Array2::from_shape_fn((height, width), |(i, _)| y_vals[i]);
        Grid { data, nodata, x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bounds() {
        let left = -2221060.;
        let bottom = 523589.;
        let right = 3181702.;
        let top = 3363319.;
        let resolution = 4000;
        let nodata = f64::NAN;

        let grid = Grid::empty_from_bounds(left, bottom, right, top, resolution, nodata);

        assert_eq!(grid.data.shape(), [709, 1350, 1]);

        assert!(grid.data[[0, 0, 0]].is_nan());
        assert_eq!(grid.x[[0, 0]], left);
        assert_eq!(grid.y[[0, 0]], bottom);

        assert!(grid.data[[708, 1349, 0]].is_nan());
        assert_eq!(grid.x[[708, 1349]], right);
        assert_eq!(grid.y[[708, 1349]], top);

        assert!(grid.data[[709 / 2, 1350 / 2, 0]].is_nan());
        assert_eq!(grid.x[[709 / 2, 1350 / 2]].round(), 482324.);
        assert_eq!(grid.y[[709 / 2, 1350 / 2]].round(), 1943454.);
    }
}
