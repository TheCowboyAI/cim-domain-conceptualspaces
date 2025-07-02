//! Command to replace dimension weights in a conceptual space (DDD-compliant)

use crate::ConceptualSpaceId;
use cim_domain::{Command, EntityId, markers::AggregateMarker};
use serde::{Deserialize, Serialize};

/// Command to replace dimension weights (removes old, adds new)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaceDimensionWeights {
    /// The space to modify
    pub space_id: ConceptualSpaceId,

    /// New weights for each dimension
    pub new_weights: Vec<f64>,

    /// Reason for replacement
    pub reason: String,
}

impl super::ConceptualSpaceCommand for ReplaceDimensionWeights {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}

impl Command for ReplaceDimensionWeights {
    type Aggregate = AggregateMarker;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.space_id.0))
    }
}
