pub mod inverse_distance_weighting;
pub mod nearest_neighbor;

pub enum InterpolationMethod {
    InverseDistanceWeighting,
    NearestNeighbor,
    // Kriging,
    // NaturalNeighbor,
}

#[derive(thiserror::Error, Debug)]
pub enum InterpolationError {
    #[error("point is outside grid")]
    PointOutsideGrid,
}
