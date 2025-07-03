//! Command handler for ConceptualSpace operations

use cim_domain::{CommandHandler, CommandEnvelope, CommandAcknowledgment, CommandStatus};
use crate::commands::{
    CreateConceptualSpace, AddConcept, AddRegion, ReplaceDimensionWeights,
};
use crate::aggregate::ConceptualSpaceAggregate;
use crate::ConceptualSpaceId;
use std::collections::HashMap;

/// Command handler for ConceptualSpace operations
pub struct ConceptualSpaceCommandHandler {
    /// In-memory storage for aggregates (in production, this would be an event store)
    aggregates: HashMap<ConceptualSpaceId, ConceptualSpaceAggregate>,
}

impl Default for ConceptualSpaceCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ConceptualSpaceCommandHandler {
    /// Create a new command handler
    pub fn new() -> Self {
        Self {
            aggregates: HashMap::new(),
        }
    }
}

impl CommandHandler<CreateConceptualSpace> for ConceptualSpaceCommandHandler {
    fn handle(&mut self, envelope: CommandEnvelope<CreateConceptualSpace>) -> CommandAcknowledgment {
        let command = envelope.command;
        let command_id = envelope.id;

        // Validate command
        if command.name.is_empty() {
            return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Space name cannot be empty".to_string()),
            };
        }

        if command.dimension_ids.is_empty() {
            return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Space must have at least one dimension".to_string()),
            };
        }

        // Check if space already exists
        if self.aggregates.contains_key(&command.space_id) {
            return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space already exists".to_string()),
            };
        }

        // Create aggregate
        let aggregate = ConceptualSpaceAggregate::new(command.name, command.dimension_ids, command.metric);

        // Store aggregate
        self.aggregates.insert(command.space_id, aggregate);

        CommandAcknowledgment {
            command_id,
            correlation_id: envelope.identity.correlation_id,
            status: CommandStatus::Accepted,
            reason: None,
        }
    }
}

impl CommandHandler<AddConcept> for ConceptualSpaceCommandHandler {
    fn handle(&mut self, envelope: CommandEnvelope<AddConcept>) -> CommandAcknowledgment {
        let command = envelope.command;
        let command_id = envelope.id;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Add concept (this validates the point)
        match aggregate.add_point(command.point.clone()) {
            Ok(_concept_id) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
}

impl CommandHandler<AddRegion> for ConceptualSpaceCommandHandler {
    fn handle(&mut self, envelope: CommandEnvelope<AddRegion>) -> CommandAcknowledgment {
        let command = envelope.command;
        let command_id = envelope.id;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Add region (this validates convexity)
        match aggregate.add_region(command.region.clone()) {
            Ok(_) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
}

impl CommandHandler<ReplaceDimensionWeights> for ConceptualSpaceCommandHandler {
    fn handle(&mut self, envelope: CommandEnvelope<ReplaceDimensionWeights>) -> CommandAcknowledgment {
        let command = envelope.command;
        let command_id = envelope.id;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Get current weights for removal event
        let _current_weights = aggregate.get_metric_weights();

        // Replace weights (this validates the weights)
        match aggregate.update_metric_weights(command.new_weights.clone()) {
            Ok(_) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id,
                correlation_id: envelope.identity.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
} 