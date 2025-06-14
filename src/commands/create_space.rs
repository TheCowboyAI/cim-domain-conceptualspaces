//! Command to create a new conceptual space

use crate::{ConceptualSpaceId, DimensionId, ConceptualMetric};
use cim_domain::{Command, EntityId, markers::AggregateMarker};
use serde::{Deserialize, Serialize};

/// Command to create a new conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConceptualSpace {
    /// ID for the new space
    pub space_id: ConceptualSpaceId,

    /// Name of the conceptual space
    pub name: String,

    /// The dimensions that define this space
    pub dimension_ids: Vec<DimensionId>,

    /// Metric structure for the space
    pub metric: ConceptualMetric,
}

impl CreateConceptualSpace {
    /// Create a new command
    pub fn new(name: String, dimension_ids: Vec<DimensionId>, metric: ConceptualMetric) -> Self {
        Self {
            space_id: ConceptualSpaceId::new(),
            name,
            dimension_ids,
            metric,
        }
    }
}

impl super::ConceptualSpaceCommand for CreateConceptualSpace {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}

impl Command for CreateConceptualSpace {
    type Aggregate = AggregateMarker;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.space_id.0))
    }
}
