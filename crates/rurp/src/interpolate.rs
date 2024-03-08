pub mod inverse_distance_weighting;
pub mod nearest_neighbor;

pub enum InterpolationMethod {
    InverseDistanceWeighting,
    NearestNeighbor,
    // Kriging,
    // NaturalNeighbor,
}
