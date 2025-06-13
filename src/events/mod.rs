//! Domain events for the Conceptual Spaces domain

mod space_created;
mod concept_added;
mod region_added;
mod weights_updated;

pub use space_created::*;
pub use concept_added::*;
pub use region_added::*;
pub use weights_updated::*;

use cim_domain::{DomainEvent, Subject};
use crate::ConceptualSpaceId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base trait for conceptual space events
pub trait ConceptualSpaceEvent: DomainEvent {
    /// Get the space ID this event relates to
    fn space_id(&self) -> ConceptualSpaceId;
}

/// All conceptual space domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptualSpaceDomainEvent {
    SpaceCreated(ConceptualSpaceCreated),
    ConceptAdded(ConceptAdded),
    RegionAdded(RegionAdded),
    WeightsUpdated(DimensionWeightsUpdated),
}

impl DomainEvent for ConceptualSpaceDomainEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::SpaceCreated(_) => "ConceptualSpaceCreated",
            Self::ConceptAdded(_) => "ConceptAdded",
            Self::RegionAdded(_) => "RegionAdded",
            Self::WeightsUpdated(_) => "DimensionWeightsUpdated",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            Self::SpaceCreated(e) => e.space_id.0,
            Self::ConceptAdded(e) => e.space_id.0,
            Self::RegionAdded(e) => e.space_id.0,
            Self::WeightsUpdated(e) => e.space_id.0,
        }
    }

    fn subject(&self) -> String {
        format!("conceptualspace.{}", self.event_type().to_lowercase())
    }
}
