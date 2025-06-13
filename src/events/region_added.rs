//! Event for region addition

use crate::{ConceptualSpaceId, ConvexRegion};
use serde::{Deserialize, Serialize};

/// Event emitted when a convex region is added to a space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionAdded {
    /// The space the region was added to
    pub space_id: ConceptualSpaceId,

    /// The region that was added
    pub region: ConvexRegion,
}
