use euclid::Transform2D;
use geo::Polygon;
use geo_rasterize::{LabelBuilder, Rasterizer};
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
        let geo_pix_transform = self.world_to_screen_transform.to_untyped();
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
    use crate::draw::write_grid_data;

    use geo::{coord, polygon};

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
    #[case(1, [0., 0., 10., 10.], 1, polygon![(x: 2., y: 2.), (x: 7., y: 2.), (x: 7., y: 7.), (x: 2., y: 7.), (x: 2., y: 2.)])]
    #[case(2, [-2221060., 523589., 3181702., 3363319.], 2000, polygon![
            coord! {x: -1951222.7162696766, y: 2354912.258633185},
            coord! {x: -2041264.2912797555, y: 2023308.6208163623},
            coord! {x: -1678141.1226114093, y: 1486297.11602201},
            coord! {x: -1665558.550028715, y: 1608253.2563886316},
            coord! {x: -1601328.9245228115, y: 1609401.5643781873},
            coord! {x: -1476039.3969683559, y: 2250299.1512043863},
            coord! {x: -1951222.7162696766, y: 2354912.258633185},
        ])]
    fn test_rasterize_polygon(
        #[case] case_number: usize,
        #[case] bounds: [f64; 4],
        #[case] resolution: usize,
        #[case] test_polygon: geo::Polygon<f64>,
    ) {
        let [left, bottom, right, top] = bounds;
        let mut grid = Grid::empty_from_bounds(f64::NAN, left, bottom, right, top, resolution);

        grid.rasterize_polygons(&[test_polygon], &[50.]);

        let path = format!("snapshots/test_rasterize_polygon_{case_number}.png");
        write_grid_data(&grid, path.as_str());
    }
}
