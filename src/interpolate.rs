use crate::grid::*;
use crate::point::*;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum InterpolationError {
    #[error("point is outside grid")]
    PointOutsideOfGrid,
}

pub fn add_point_data_to_grid(
    point_data: &[Point],
    grid: &mut Grid,
) -> Result<(), InterpolationError> {
    let x_width = grid.x[[0, grid.x.shape()[1] - 1]] - grid.x[[0, 0]];
    let x_point_width = x_width / grid.x.shape()[1] as f64;

    let y_height = grid.y[[grid.y.shape()[0] - 1, 0]] - grid.y[[0, 0]];
    let y_point_height = y_height / grid.y.shape()[0] as f64;

    let mut points_grid_points_map: HashMap<(usize, usize), Vec<&Point>> = HashMap::new();

    for point in point_data {
        let point_grid_pos_x = ((point.x - grid.x[[0, 0]]) / x_point_width).floor() as isize;
        let point_grid_pos_y = ((point.y - grid.y[[0, 0]]) / y_point_height).floor() as isize;
        if point_grid_pos_y < 0
            || point_grid_pos_y >= grid.data.shape()[0] as isize
            || point_grid_pos_x < 0
            || point_grid_pos_x >= grid.data.shape()[1] as isize
        {
            return Err(InterpolationError::PointOutsideOfGrid);
        }
        points_grid_points_map
            .entry((point_grid_pos_y as usize, point_grid_pos_x as usize))
            .and_modify(|e| e.push(point))
            .or_insert(vec![point]);
    }
    for ((index_y, index_x), points) in points_grid_points_map {
        grid.data[[index_y, index_x, 0]] =
            points.iter().map(|&p| p.values[0]).sum::<f64>() / points.len() as f64;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;
    use rstest::rstest;

    #[rstest]
    #[case(0., 0., 10., 10., 1, 30)]
    #[case(-100000., 100000., 100000., 200000., 1000, 9999)]
    #[case::conus(-2221060., 523589., 3181702., 3363319., 4000, 99999)]
    fn test_it_add_point_data_to_grid(
        #[case] left: f64,
        #[case] bottom: f64,
        #[case] right: f64,
        #[case] top: f64,
        #[case] resolution: usize,
        #[case] num_stub_stations: usize,
    ) {
        let x_iter =
            Array::linspace(left, right - resolution as f64 / 2., num_stub_stations).into_iter();
        let y_iter =
            Array::linspace(bottom, top - resolution as f64 / 2., num_stub_stations).into_iter();
        let value_iter = Array::linspace(0., 100., num_stub_stations)
            .into_iter()
            .map(|v| vec![v]);
        let stub_point_data = x_iter
            .zip(y_iter)
            .zip(value_iter)
            .map(|((x, y), values)| Point::new(x, y, values))
            .collect::<Vec<Point>>();
        let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);

        add_point_data_to_grid(&stub_point_data, &mut grid).unwrap();

        insta::assert_debug_snapshot!(grid);
    }

    #[rstest]
    #[case::x_out_of_bounds(Point::new(2., -1., vec![1.]))]
    #[case::y_out_of_bounds(Point::new(11.0, 2., vec![1.]))]
    fn test_it_returns_error_if_points_are_outside_grid(#[case] err_point: Point) {
        let err_points = vec![err_point];
        let mut grid = Grid::empty_from_bounds(f64::NAN, 0., 0., 10., 10., 1);

        let result = add_point_data_to_grid(&err_points, &mut grid);
        assert!(result.is_err())
    }
}
