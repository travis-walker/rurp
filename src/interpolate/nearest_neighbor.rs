use crate::grid::Grid;
use geo::{LineString, Polygon};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

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

    grid.rasterize_polygons(&polygons, point_z)
}

pub fn apply_nearest_neighbor_interpolation(x: &[f64], y: &[f64], z: &[f64], grid: &mut Grid) {
    let (left, bottom, right, top) = grid.bounds();
    let points = x
        .par_iter()
        .zip(y.par_iter())
        .map(|(p_x, p_y)| (*p_x, *p_y))
        .collect::<Vec<_>>();
    let voronoi = VoronoiDiagram::from_tuple(&(left, bottom), &(right, top), &points).unwrap();
    voronoi_to_grid(&voronoi, z, grid);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    use rstest::rstest;

    fn build_stub_point_data(
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        point_count: usize,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut rng = StdRng::seed_from_u64(43691);
        let x_range = Uniform::new(left, right);
        let y_range = Uniform::new(bottom, top);
        let z_range = Uniform::new(-5.0f64, 120.0f64);
        (
            (0..point_count).map(|_| rng.sample(x_range)).collect(),
            (0..point_count).map(|_| rng.sample(y_range)).collect(),
            (0..point_count).map(|_| rng.sample(z_range)).collect(),
        )
    }

    mod test_apply_nearest_neighbor_interpolation {
        use super::*;
        use crate::draw::draw_grid_data;

        #[rstest]
        #[case(-10., 0., 10., 10., 1, 10)]
        #[case(-2221060., 523589., 3181702., 3363319., 4000, 8000)]
        #[case(-2221060., 523589., 3181702., 3363319., 2000, 16000)]
        fn test_it_interpolates_as_expected(
            #[case] left: f64,
            #[case] bottom: f64,
            #[case] right: f64,
            #[case] top: f64,
            #[case] resolution: usize,
            #[case] point_count: usize,
        ) {
            let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
            let (x, y, z) = build_stub_point_data(left, bottom, right, top, point_count);

            apply_nearest_neighbor_interpolation(&x, &y, &z, &mut grid);
            draw_grid_data(
                &grid,
                "test_images/test_apply_nearest_neighbor_interpolation.png",
            );
        }
    }
}
