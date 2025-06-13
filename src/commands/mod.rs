//! Commands for the Conceptual Spaces domain

mod create_space;
mod add_concept;
mod add_region;
mod update_weights;

pub use create_space::*;
pub use add_concept::*;
pub use add_region::*;
pub use update_weights::*;

use cim_domain::{Command, EntityId};
use crate::{ConceptualSpaceAggregate, ConceptualSpaceId};

/// Base trait for conceptual space commands
pub trait ConceptualSpaceCommand: Command {
    /// Get the space ID this command targets
    fn space_id(&self) -> ConceptualSpaceId;
}

impl<T: ConceptualSpaceCommand> Command for T {
    type Aggregate = ConceptualSpaceAggregate;

    fn aggregate_id(&self) -> Option<EntityId<Self::Aggregate>> {
        Some(EntityId::from(self.space_id().0))
    }
}
