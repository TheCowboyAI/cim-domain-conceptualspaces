//! Event for conceptual space creation

use crate::{ConceptualSpaceId, DimensionId, ConceptualMetric};
use serde::{Deserialize, Serialize};

/// Event emitted when a conceptual space is created
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualSpaceCreated {
    /// ID of the created space
    pub space_id: ConceptualSpaceId,

    /// Name of the conceptual space
    pub name: String,

    /// The dimensions that define this space
    pub dimension_ids: Vec<DimensionId>,

    /// Metric structure for the space
    pub metric: ConceptualMetric,
}
