use crate::grid::Grid;
use geo::{LineString, Polygon};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use voronator::{delaunator::Point, VoronoiDiagram};

fn voronoi_to_grid(voronoi: &VoronoiDiagram<Point>, point_z: &[f64], grid: &mut Grid) {
    let polygons = voronoi
        .cells()
        .iter()
        .map(|cell| {
            let exterior = cell
                .points()
                .iter()
                .map(|x| (x.x, x.y))
                .collect::<LineString>();
            Polygon::new(exterior, vec![])
        })
        .collect::<Vec<Polygon>>();

    grid.rasterize_polygons(&polygons, point_z);
}

pub fn apply_nearest_neighbor_interpolation(x: &[f64], y: &[f64], z: &[f64], grid: &mut Grid) {
    let (left, bottom, right, top) = grid.bounds();
    let points = x
        .par_iter()
        .zip(y.par_iter())
        .map(|(p_x, p_y)| (*p_x, *p_y))
        .collect::<Vec<_>>();
    let voronoi = VoronoiDiagram::from_tuple(&(left, bottom), &(right, top), &points)
        .expect("unable to build voronoi");
    voronoi_to_grid(&voronoi, z, grid);
}
