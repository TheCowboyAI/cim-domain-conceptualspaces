//! Command to update dimension weights in a conceptual space

use crate::ConceptualSpaceId;
use serde::{Deserialize, Serialize};

/// Command to update dimension weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionWeights {
    /// The space to update
    pub space_id: ConceptualSpaceId,

    /// New weight values
    pub weights: Vec<f64>,
}

impl super::ConceptualSpaceCommand for UpdateDimensionWeights {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}
