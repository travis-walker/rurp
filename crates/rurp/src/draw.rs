use crate::{equivalent, grid::Grid, normalize};
use image::ImageBuffer;

use std::fs;

/// Write grid data to an image file.
/// # Errors
/// Returns an error if the image file cannot be written.
pub fn write_grid_data(grid: &Grid, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let height = grid.height();
    let width = grid.width();
    let grid_data = grid.data();
    let nodata = grid.nodata();

    let data_domain = grid_data
        .iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |acc, &v| {
            if equivalent(&v, &nodata) {
                acc
            } else {
                (acc.0.min(v), acc.1.max(v))
            }
        });

    let pixel_domain = (1.0, f64::from(u16::MAX - 1));

    let img = ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
        let cell_value = grid_data[[height - 1 - y as usize, x as usize, 0]];
        let pixel_value = if equivalent(&cell_value, &nodata) {
            0
        } else {
            normalize(cell_value, &data_domain, &pixel_domain).round() as u16
        };
        image::Luma([pixel_value])
    });
    let _ = fs::remove_file(path);
    img.save(path)?;
    Ok(())
}
