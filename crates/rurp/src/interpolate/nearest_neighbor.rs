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

pub fn apply_nearest_neighbor_interpolation(grid: &mut Grid, points: &[Point]) {
    let (left, bottom, right, top) = grid.bounds();

    let voronoi_points: Vec<_> = points.par_iter().map(std::convert::Into::into).collect();
    let polygon_labels: Vec<_> = points.par_iter().map(|point| point.values[0]).collect();

    let voronoi =
        VoronoiDiagram::from_tuple(&(left, bottom), &(right, top), &voronoi_points).unwrap();

    voronoi_to_grid(&voronoi, &polygon_labels, grid);
}
