use crate::grid::Grid;
use image::ImageBuffer;
use std::cmp::Ordering;
use std::fs;

pub fn wite_grid_data(grid: &Grid, path: &str) {
    let height = grid.data.shape()[0];
    let width = grid.data.shape()[1];
    let domain_max = grid
        .data
        .iter()
        .max_by(|&a, &b| a.partial_cmp(b).unwrap_or(Ordering::Less))
        .unwrap();

    let img = ImageBuffer::from_fn(
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        |x, y: u32| {
            let pixel_value = { grid.data[[y as usize, x as usize, 0]] / domain_max * 255. } as u8;
            image::Luma([pixel_value])
        },
    );
    let _ = fs::remove_file(path);
    img.save(format!("src/{path}")).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_wite_grid_data {
        use super::*;

        #[test]
        fn test_draw_a_grid() {
            let (left, bottom, right, top, resolution) = (0., 0., 500., 250., 1);
            let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
            grid.data
                .indexed_iter_mut()
                .for_each(|((y, x, _), value)| *value = { x + y } as f64);

            wite_grid_data(&grid, "snapshots/test_plot_a_grid-1.png");
        }
    }
}
