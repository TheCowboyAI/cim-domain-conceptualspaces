//! Command to update dimension weights in a conceptual space

use crate::ConceptualSpaceId;
use cim_domain::{Command, EntityId, markers::AggregateMarker};
use serde::{Deserialize, Serialize};

/// Command to update dimension weights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDimensionWeights {
    /// The space to update
    pub space_id: ConceptualSpaceId,

    /// New weights for each dimension
    pub weights: Vec<f64>,
}

impl super::ConceptualSpaceCommand for UpdateDimensionWeights {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}

impl Command for UpdateDimensionWeights {
    type Aggregate = AggregateMarker;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.space_id.0))
    }
}
