use crate::grid::Grid;
use image::ImageBuffer;

use std::fs;

pub fn write_grid_data(grid: &Grid, path: &str) {
    let height = grid.height;
    let width = grid.width;
    let domain_max = grid
        .data
        .iter()
        .filter(|v| v.is_finite())
        .max_by(|l, r| l.total_cmp(r))
        .unwrap();

    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let pixel_value = {
            let cell_value = grid.data[[grid.height - 1 - y as usize, x as usize, 0]];
            if cell_value.is_finite() {
                255 - (cell_value / domain_max * 255.) as u8
            } else {
                255
            }
        };
        image::Luma([pixel_value])
    });
    let _ = fs::remove_file(path);
    img.save(path).unwrap();
}
