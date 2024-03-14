pub mod inverse_distance_weighting;
pub mod inverse_distance_weighting_global;
pub mod natural_neighbor;
pub mod nearest_neighbor;

pub enum InterpolationMethod {
    InverseDistanceWeighting,
    InverseDistanceWeightingGlobal,
    NaturalNeighbor,
    NearestNeighbor,
    // Kriging,
}
