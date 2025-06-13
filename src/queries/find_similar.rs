//! Query to find similar concepts in a conceptual space

use crate::{ConceptualSpaceId, ConceptualPoint};
use cim_domain::Query;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Query to find concepts similar to a given concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSimilarConcepts {
    /// The space to search in
    pub space_id: ConceptualSpaceId,

    /// The reference point
    pub reference: ConceptualPoint,

    /// Maximum number of results
    pub limit: usize,

    /// Maximum distance threshold
    pub max_distance: Option<f64>,
}

/// Result of finding similar concepts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarConcepts {
    /// List of similar points with their distances
    pub points: Vec<(Uuid, ConceptualPoint, f64)>,
}

impl Query for FindSimilarConcepts {
    type Result = SimilarConcepts;
}
