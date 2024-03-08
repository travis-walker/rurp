use euclid::Transform2D;
use geo::Polygon;
use geo_rasterize::{LabelBuilder, Rasterizer};
use ndarray::{prelude::*, ArrayViewMut3};

pub struct WorldSpace;
pub struct ScreenSpace;

pub type Bounds = (f64, f64, f64, f64);

#[derive(Debug)]
pub struct Grid {
    data: Array3<f64>,
    x: Array2<f64>,
    y: Array2<f64>,
    bounds: Bounds,
    height: usize,
    width: usize,
    world_height: f64,
    world_width: f64,
    world_to_screen_transform: Transform2D<f64, WorldSpace, ScreenSpace>,
    screen_to_world_transform: Transform2D<f64, ScreenSpace, WorldSpace>,
    nodata: f64,
}
impl Grid {
    #[must_use]
    pub fn data(&self) -> ArrayView3<f64> {
        self.data.view()
    }

    pub fn data_mut(&mut self) -> ArrayViewMut3<f64> {
        self.data.view_mut()
    }

    #[must_use]
    pub fn x(&self) -> ArrayView2<f64> {
        self.x.view()
    }

    #[must_use]
    pub fn y(&self) -> ArrayView2<f64> {
        self.y.view()
    }

    #[must_use]
    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn world_height(&self) -> f64 {
        self.world_height
    }

    #[must_use]
    pub fn world_width(&self) -> f64 {
        self.world_width
    }

    #[must_use]
    pub fn screen_to_world_transform(&self) -> Transform2D<f64, ScreenSpace, WorldSpace> {
        self.screen_to_world_transform
    }

    #[must_use]
    pub fn world_to_screen_transform(&self) -> Transform2D<f64, WorldSpace, ScreenSpace> {
        self.world_to_screen_transform
    }

    #[must_use]
    pub fn nodata(&self) -> f64 {
        self.nodata
    }
}

impl Grid {
    #[must_use]
    pub fn empty_from_bounds(bounds: Bounds, resolution: usize, nodata: f64) -> Self {
        let (left, bottom, right, top) = bounds;

        let world_height = top - bottom;
        let world_width = right - left;

        let height = (world_height / resolution as f64).ceil() as usize;
        let width = (world_width / resolution as f64).ceil() as usize;

        let data = Array3::from_elem((height, width, 1), nodata);

        let x_vals = Array1::linspace(left, right, width);
        let x = Array2::from_shape_fn((height, width), |(_, j)| x_vals[j]);

        let y_vals = Array1::linspace(bottom, top, height);
        let y = Array2::from_shape_fn((height, width), |(i, _)| y_vals[i]);

        let world_to_screen_transform: Transform2D<f64, WorldSpace, ScreenSpace> =
            Transform2D::translation(-left, -bottom)
                .then_scale(width as f64 / world_width, height as f64 / world_height);
        let screen_to_world_transform = world_to_screen_transform.inverse().unwrap();

        Grid {
            data,
            x,
            y,
            bounds,
            height,
            width,
            world_height,
            world_width,
            world_to_screen_transform,
            screen_to_world_transform,
            nodata,
        }
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
