use crate::grid::*;
use crate::point::Point;
#[allow(dead_code)]
pub fn apply_nearest_neighbor_interpolation(grid: &mut Grid) {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;
    use rstest::rstest;

    #[allow(dead_code)]
    fn build_stub_point_data(
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        point_distance: f64,
        point_count: usize,
    ) -> Vec<Point> {
        let x_iter = Array::linspace(left, right - point_distance / 2., point_count).into_iter();
        let y_iter = Array::linspace(bottom, top - point_distance / 2., point_count).into_iter();
        let value_iter = Array::linspace(0., 100., point_count)
            .into_iter()
            .map(|v| vec![v]);

        x_iter
            .zip(y_iter)
            .zip(value_iter)
            .map(|((x, y), values)| Point::new(x, y, values))
            .collect::<Vec<Point>>()
    }

    mod test_apply_nearest_neighbor_interpolation {
        use super::*;

        #[rstest]
        fn test_1() {
            let (left, bottom, right, top, resolution) = (-10., 0., 10., 10., 1);
            let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
            grid.data
                .indexed_iter_mut()
                .for_each(|((y, x, _), cell)| *cell = (y + x) as f64);

            apply_nearest_neighbor_interpolation(&mut grid);

            // insta::assert_debug_snapshot!(grid.data);
        }
    }
}
