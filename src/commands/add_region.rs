//! Command to add a region to a conceptual space

use crate::{ConceptualSpaceId, ConvexRegion};
use cim_domain::{Command, EntityId, markers::AggregateMarker};
use serde::{Deserialize, Serialize};

/// Command to add a region to a conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddRegion {
    /// The space to add the region to
    pub space_id: ConceptualSpaceId,

    /// The region to add
    pub region: ConvexRegion,
}

impl super::ConceptualSpaceCommand for AddRegion {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}

impl Command for AddRegion {
    type Aggregate = AggregateMarker;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.space_id.0))
    }
}
