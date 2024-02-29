use crate::grid::Grid;
use geo::Polygon;
use geo_rasterize::LabelBuilder;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

// TODO: make generic raster method on grid
fn voronoi_to_grid(voronoi: &VoronoiDiagram<Point>, point_z: &[f64], grid: &mut Grid) {
    let grid_shape = grid.data.shape();

    let mut rasterizer = LabelBuilder::background(grid.nodata)
        .width(grid_shape[1])
        .height(grid_shape[0])
        .geo_to_pix(grid.world_to_screen_transform())
        .build()
        .unwrap();
    voronoi
        .cells()
        .iter()
        .zip(point_z.iter())
        .for_each(|(cell, z)| {
            let p = cell
                .points()
                .par_iter()
                .map(|x| (x.x, x.y))
                .collect::<Vec<_>>();
            let polygon = Polygon::new(p.into(), vec![]);
            rasterizer.rasterize(&polygon, *z).unwrap();
        });
    grid.data = rasterizer
        .finish()
        .into_shape((grid_shape[0], grid_shape[1], 1))
        .unwrap();
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

        #[rstest]
        #[case(-10., 0., 10., 10., 1, 10)]
        #[case(-2221060., 523589., 3181702., 3363319., 4000, 8000)]
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

            dbg!(&grid.data);
            // panic!()

            // insta::assert_debug_snapshot!(grid.data);
        }
    }
}
