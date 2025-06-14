//! Command to add a concept to a conceptual space

use crate::{ConceptualSpaceId, ConceptualPoint};
use cim_domain::{Command, EntityId, markers::AggregateMarker};
use serde::{Deserialize, Serialize};

/// Command to add a concept to a conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddConcept {
    /// The space to add the concept to
    pub space_id: ConceptualSpaceId,

    /// The point representing the concept
    pub point: ConceptualPoint,
}

impl super::ConceptualSpaceCommand for AddConcept {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}

impl Command for AddConcept {
    type Aggregate = AggregateMarker;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from_uuid(self.space_id.0))
    }
}
