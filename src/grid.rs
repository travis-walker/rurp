use euclid::Transform2D;
use geo::Polygon;
use geo_rasterize::LabelBuilder;
use geo_rasterize::Rasterizer;
use ndarray::{Array1, Array2, Array3};
pub struct WorldSpace;
pub struct ScreenSpace;

#[derive(Debug)]
pub struct Grid {
    pub nodata: f64,
    pub data: Array3<f64>,
    pub x: Array2<f64>,
    pub y: Array2<f64>,
    pub height: usize,
    pub width: usize,
    pub world_height: f64,
    pub world_width: f64,
    pub world_to_screen_transform: Transform2D<f64, WorldSpace, ScreenSpace>,
    pub screen_to_world_transform: Transform2D<f64, ScreenSpace, WorldSpace>,
}

impl Grid {
    pub fn empty_from_bounds(
        nodata: f64,
        left: f64,
        bottom: f64,
        right: f64,
        top: f64,
        resolution: usize,
    ) -> Self {
        let width = ((right - left) / resolution as f64).ceil() as usize;
        let height = ((top - bottom) / resolution as f64).ceil() as usize;

        let data = Array3::from_elem((height, width, 1), nodata);

        let x_vals = Array1::linspace(left, right, width);
        let x = Array2::from_shape_fn((height, width), |(_, j)| x_vals[j]);

        let y_vals = Array1::linspace(bottom, top, height);
        let y = Array2::from_shape_fn((height, width), |(i, _)| y_vals[i]);

        let world_height = top - bottom;
        let world_width = right - left;

        let world_to_screen_transform: Transform2D<f64, WorldSpace, ScreenSpace> =
            Transform2D::translation(-left, -bottom)
                .then_scale(width as f64 / world_width, height as f64 / world_height);
        let screen_to_world_transform = world_to_screen_transform.inverse().unwrap();
        Grid {
            nodata,
            data,
            x,
            y,
            height,
            width,
            world_height,
            screen_to_world_transform,
            world_to_screen_transform,
            world_width,
        }
    }

    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        (
            self.x[[0, 0]],
            self.y[[0, 0]],
            self.x[[0, self.width - 1]],
            self.y[[self.height - 1, 0]],
        )
    }
    pub fn rasterize_polygons(&mut self, polygons: &[Polygon<f64>], z: &[f64]) {
        let mut rasterizer = self.build_default_rasterizer();

        polygons.iter().zip(z).for_each(|(polygon, z)| {
            rasterizer.rasterize(polygon, *z).unwrap();
        });

        self.data = rasterizer
            .finish()
            .into_shape((self.height, self.width, 1))
            .unwrap();
    }
}

impl Grid {
    fn build_default_rasterizer(&self) -> Rasterizer<f64> {
        let geo_pix_transform = Transform2D::translation(-self.x[[0, 0]], -self.y[[0, 0]])
            .then_scale(
                self.width as f64 / self.world_width,
                self.height as f64 / self.world_height,
            );
        LabelBuilder::background(self.nodata)
            .width(self.width)
            .height(self.height)
            .geo_to_pix(geo_pix_transform)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use core::f64;
    use geo::polygon;

    use super::*;
    use rstest::rstest;

    fn equivalent(left: f64, right: f64) -> bool {
        left.is_nan() && right.is_nan() || left == right
    }

    #[rstest]
    #[case(4000, 710, 1351, f64::NAN)]
    #[case(8000, 355, 676, f64::NEG_INFINITY)]
    #[case(1200, 2367, 4503, f64::MAX)]
    fn test_from_bounds(
        #[case] resolution: usize,
        #[case] expected_height: usize,
        #[case] expected_width: usize,
        #[case] nodata: f64,
    ) {
        let left = -2221060.;
        let bottom = 523589.;
        let right = 3181702.;
        let top = 3363319.;

        let grid = Grid::empty_from_bounds(nodata, left, bottom, right, top, resolution);

        assert_eq!(grid.data.shape(), [expected_height, expected_width, 1]);

        assert!(equivalent(grid.data[[0, 0, 0]], nodata));
        assert_eq!(grid.x[[0, 0]], left);
        assert_eq!(grid.y[[0, 0]], bottom);

        let max_y = expected_height - 1;
        let max_x = expected_width - 1;
        assert!(equivalent(grid.data[[max_y, max_x, 0]], nodata));
        assert_eq!(grid.x[[max_y, max_x]], right);
        assert_eq!(grid.y[[max_y, max_x]], top);

        let mid_y = expected_height / 2;
        let mid_x = expected_width / 2;
        assert!(equivalent(grid.data[[mid_y, mid_x, 0]], nodata));
        assert!((grid.x[[mid_y, mid_x]] - ((right + left) / 2.)).abs() < resolution as f64);
        assert!((grid.y[[mid_y, mid_x]] - ((top + bottom) / 2.)).abs() < resolution as f64);
    }

    #[rstest]
    #[case(0., 0., 10., 15., 1, 15, 10)]
    #[case(-2221060., 523589., 3181702., 3363319., 8000, 355, 676)]
    fn test_properties(
        #[case] left: f64,
        #[case] bottom: f64,
        #[case] right: f64,
        #[case] top: f64,
        #[case] resolution: usize,
        #[case] expected_height: usize,
        #[case] expected_width: usize,
    ) {
        let grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);

        assert_eq!(grid.width, expected_width);
        assert_eq!(grid.height, expected_height);
        assert_eq!(grid.bounds(), (left, bottom, right, top));

        assert_eq!(grid.world_width, right - left);
        assert_eq!(grid.world_height, top - bottom);
    }
    #[rstest]
    #[case(-1., -1., 10., 15., 1, [0., 0.].into(), [-1., -1.].into())]
    #[case(-2221060., 523589., 3181702., 3363319., 8000, [0., 0.].into(), [-2221060., 523589.].into())]
    fn test_transform(
        #[case] left: f64,
        #[case] bottom: f64,
        #[case] right: f64,
        #[case] top: f64,
        #[case] resolution: usize,
        #[case] test_point_screen: euclid::Point2D<f64, ScreenSpace>,
        #[case] test_point_world: euclid::Point2D<f64, WorldSpace>,
    ) {
        let grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);

        let s_w_transform = grid.screen_to_world_transform;

        let sw_transformed_point = s_w_transform.transform_point(test_point_screen);
        assert_eq!(sw_transformed_point, test_point_world);

        let w_s_transform = grid.world_to_screen_transform;

        let ws_transformed_point = w_s_transform.transform_point(test_point_world);
        assert_eq!(ws_transformed_point, test_point_screen);
    }

    #[rstest]
    #[case(0., 0., 10., 10., 1, polygon![(x: 1., y: 1.), (x: 9., y: 1.), (x: 9., y: 9.), (x: 1., y: 9.)], 10.)]
    fn test_rasterize_polygon(
        #[case] left: f64,
        #[case] bottom: f64,
        #[case] right: f64,
        #[case] top: f64,
        #[case] resolution: usize,
        #[case] test_polygon: geo::Polygon<f64>,
        #[case] z: f64,
    ) {
        let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);
        grid.rasterize_polygons(&[test_polygon], &[z]);
    }
}
