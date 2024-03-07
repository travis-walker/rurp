use euclid::Transform2D;
use geo::Polygon;
use geo_rasterize::{LabelBuilder, Rasterizer};
use ndarray::{Array1, Array2, Array3};

pub struct WorldSpace;
pub struct ScreenSpace;

pub type Bounds = (f64, f64, f64, f64);

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

    pub fn bounds(&self) -> Bounds {
        (
            self.x[[0, 0]],
            self.y[[0, 0]],
            self.x[[0, self.width - 1]],
            self.y[[self.height - 1, 0]],
        )
    }
    pub fn rasterize_polygons(&mut self, polygons: &[Polygon<f64>], polygon_labels: &[f64]) {
        let mut rasterizer = self.build_default_rasterizer();

        polygons
            .iter()
            .zip(polygon_labels)
            .for_each(|(polygon, polygon_label)| {
                rasterizer.rasterize(polygon, *polygon_label).unwrap();
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
