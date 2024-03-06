pub enum InterpolationMethod {
    Linear,
    NearestNeighbor,
    // GaussianProcessRegression,
    // InverseDistanceWeighting,
    // NaturalNeighbor,
}
#[derive(thiserror::Error, Debug)]
pub enum InterpolationError {
    #[error("point is outside grid")]
    PointOutsideGrid,
}
pub mod add_to_grid;
pub mod inverse_distance_weighting;
pub mod nearest_neighbor;
