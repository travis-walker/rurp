use std::error::Error;

use crate::grid::Grid;
use crate::point::Point;
use geo::{LineString, Polygon};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use voronator::{delaunator, VoronoiDiagram};

fn voronoi_to_grid(
    voronoi: &VoronoiDiagram<delaunator::Point>,
    polygon_labels: &[f64],
    grid: &mut Grid,
) {
    let polygons = voronoi
        .cells()
        .par_iter()
        .map(|cell| {
            let exterior = cell
                .points()
                .iter()
                .map(|point| (point.x, point.y))
                .collect::<LineString>();
            Polygon::new(exterior, vec![])
        })
        .collect::<Vec<Polygon>>();

    grid.rasterize_polygons(&polygons, polygon_labels);
}

pub fn interpolate(grid: &mut Grid, points: &[Point]) -> Result<(), Box<dyn Error>> {
    if points.is_empty() {
        return Err("No points to interpolate".into());
    }

    let (left, bottom, right, top) = grid.bounds().into();

    let voronoi_points: Vec<_> = points.par_iter().map(std::convert::Into::into).collect();
    let polygon_labels: Vec<_> = points.par_iter().map(|point| point.values[0]).collect();

    if let Some(voronoi) =
        VoronoiDiagram::from_tuple(&(left, bottom), &(right, top), &voronoi_points)
    {
        voronoi_to_grid(&voronoi, &polygon_labels, grid);
        Ok(())
    } else {
        Err("Error building voronoi diagram".into())
    }
}
