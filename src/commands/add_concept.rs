//! Command to add a concept to a conceptual space

use crate::{ConceptualSpaceId, ConceptualPoint};
use serde::{Deserialize, Serialize};

/// Command to add a concept to a conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddConcept {
    /// The space to add the concept to
    pub space_id: ConceptualSpaceId,

    /// The point to add
    pub point: ConceptualPoint,
}

impl super::ConceptualSpaceCommand for AddConcept {
    fn space_id(&self) -> ConceptualSpaceId {
        self.space_id
    }
}
