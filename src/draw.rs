use crate::grid::Grid;
use image::ImageBuffer;
use std::fs;

pub fn draw_grid_data(grid: &Grid, path: &str) {
    let height = grid.data.shape()[0];
    let width = grid.data.shape()[1];
    let domain_max = grid
        .data
        .iter()
        .max_by(|&a, &b| a.partial_cmp(b).unwrap())
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
    img.save(path).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    mod test_draw_grid_data {
        use std::fs::create_dir_all;

        use super::*;

        #[test]
        fn test_draw_a_grid() {
            let (left, bottom, right, top, resolution) = (0., 0., 500., 250., 1);
            let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
            grid.data
                .indexed_iter_mut()
                .for_each(|((y, x, _), value)| *value = { x + y } as f64);
            let test_img_dir = "test_images/test_draw_grid_data";
            create_dir_all(test_img_dir).unwrap();
            let test_img_path = format!("{}/test_plot_a_grid.png", test_img_dir);

            draw_grid_data(&grid, &test_img_path);
        }
    }
}
