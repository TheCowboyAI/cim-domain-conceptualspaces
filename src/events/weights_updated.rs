//! Event for dimension weight updates

use crate::ConceptualSpaceId;
use serde::{Deserialize, Serialize};

/// Event emitted when dimension weights are updated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionWeightsUpdated {
    /// The space whose weights were updated
    pub space_id: ConceptualSpaceId,

    /// The old weight values
    pub old_weights: Vec<f64>,

    /// The new weight values
    pub new_weights: Vec<f64>,
}
