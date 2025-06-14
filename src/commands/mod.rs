//! Commands for the Conceptual Spaces domain

mod create_space;
mod add_concept;
mod add_region;
mod update_weights;

pub use create_space::*;
pub use add_concept::*;
pub use add_region::*;
pub use update_weights::*;

use crate::ConceptualSpaceId;

/// Base trait for conceptual space commands
pub trait ConceptualSpaceCommand {
    /// Get the space ID this command targets
    fn space_id(&self) -> ConceptualSpaceId;
}
