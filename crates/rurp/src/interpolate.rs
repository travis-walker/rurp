pub mod inverse_distance_weighting;
pub mod inverse_distance_weighting_global;
pub mod nearest_neighbor;

pub enum InterpolationMethod {
    InverseDistanceWeighting,
    NearestNeighbor,
    // Kriging,
    // NaturalNeighbor,
}
