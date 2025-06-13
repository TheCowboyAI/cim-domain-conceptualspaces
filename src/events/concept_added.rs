//! Event for concept addition

use crate::{ConceptualSpaceId, ConceptualPoint};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Event emitted when a concept is added to a space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptAdded {
    /// The space the concept was added to
    pub space_id: ConceptualSpaceId,

    /// ID assigned to the concept
    pub concept_id: Uuid,

    /// The point that was added
    pub point: ConceptualPoint,
}
