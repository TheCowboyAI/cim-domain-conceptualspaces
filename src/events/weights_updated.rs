//! Events for dimension weight changes (following DDD event sourcing principles)

use crate::ConceptualSpaceId;
use serde::{Deserialize, Serialize};

/// Event emitted when dimension weights are removed (first part of replacement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionWeightsRemoved {
    /// The space whose weights were removed
    pub space_id: ConceptualSpaceId,

    /// The removed weight values (for audit trail)
    pub removed_weights: Vec<f64>,

    /// Reason for removal
    pub reason: String,
}

/// Event emitted when new dimension weights are added (second part of replacement)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionWeightsAdded {
    /// The space where weights were added
    pub space_id: ConceptualSpaceId,

    /// The new weight values
    pub weights: Vec<f64>,

    /// Reason for addition
    pub reason: String,
}
