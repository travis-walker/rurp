pub enum InterpolationMethod {
    NearestNeighbor,
    // NaturalNeighbor,
    // InverseDistanceWeighting,
    // GaussianProcessRegression,
}
#[derive(thiserror::Error, Debug)]
pub enum InterpolationError {
    #[error("point is outside grid")]
    PointOutsideOfGrid,
}
pub mod add_to_grid;
pub mod nearest_neighbor;
