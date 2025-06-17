//! Command handler for ConceptualSpace aggregate

use crate::{
    ConceptualSpaceAggregate, ConceptualSpaceId,
    CreateConceptualSpace, AddConcept, AddRegion, ReplaceDimensionWeights,
};
use cim_domain::{
    CommandHandler, CommandEnvelope, CommandAcknowledgment, CommandStatus
};
use std::collections::HashMap;

/// Command handler for ConceptualSpace operations
pub struct ConceptualSpaceCommandHandler {
    /// In-memory storage for aggregates (in production, this would be an event store)
    aggregates: HashMap<ConceptualSpaceId, ConceptualSpaceAggregate>,
}

impl ConceptualSpaceCommandHandler {
    /// Create a new command handler
    pub fn new() -> Self {
        Self {
            aggregates: HashMap::new(),
        }
    }

    /// Get an aggregate by ID (for testing and queries)
    pub fn get_aggregate(&self, space_id: &ConceptualSpaceId) -> Option<&ConceptualSpaceAggregate> {
        self.aggregates.get(space_id)
    }

    /// Get all aggregates (for testing)
    pub fn get_all_aggregates(&self) -> &HashMap<ConceptualSpaceId, ConceptualSpaceAggregate> {
        &self.aggregates
    }
}

impl Default for ConceptualSpaceCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandHandler<CreateConceptualSpace> for ConceptualSpaceCommandHandler {
    /// Handle the CreateConceptualSpace command
    ///
    /// ```mermaid
    /// graph TD
    ///     A[CreateConceptualSpace] --> B[Validate Command]
    ///     B --> C[Create Aggregate]
    ///     C --> D[Emit SpaceCreated Event]
    ///     D --> E[Store Aggregate]
    /// ```
    fn handle(&mut self, envelope: CommandEnvelope<CreateConceptualSpace>) -> CommandAcknowledgment {
        let command = envelope.command;

        // Validate command
        if command.name.is_empty() {
            return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Space name cannot be empty".to_string()),
            };
        }

        if command.dimension_ids.is_empty() {
            return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Space must have at least one dimension".to_string()),
            };
        }

        // Check if space already exists
        if self.aggregates.contains_key(&command.space_id) {
            return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space already exists".to_string()),
            };
        }

        // Create new aggregate
        let aggregate = ConceptualSpaceAggregate::new(
            command.name.clone(),
            command.dimension_ids.clone(),
            command.metric.clone(),
        );

        // Store aggregate
        self.aggregates.insert(command.space_id, aggregate);

        CommandAcknowledgment {
            command_id: envelope.id,
            correlation_id: envelope.correlation_id,
            status: CommandStatus::Accepted,
            reason: None,
        }
    }
}

impl CommandHandler<AddConcept> for ConceptualSpaceCommandHandler {
    /// Handle the AddConcept command
    ///
    /// ```mermaid
    /// graph TD
    ///     A[AddConcept] --> B[Load Aggregate]
    ///     B --> C[Validate Concept]
    ///     C --> D[Add to Space]
    ///     D --> E[Emit ConceptAdded Event]
    ///     E --> F[Update Aggregate]
    /// ```
    fn handle(&mut self, envelope: CommandEnvelope<AddConcept>) -> CommandAcknowledgment {
        let command = envelope.command;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Add concept (this validates the point)
        match aggregate.add_point(command.point.clone()) {
            Ok(_concept_id) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
}

impl CommandHandler<AddRegion> for ConceptualSpaceCommandHandler {
    /// Handle the AddRegion command
    ///
    /// ```mermaid
    /// graph TD
    ///     A[AddRegion] --> B[Load Aggregate]
    ///     B --> C[Validate Region]
    ///     C --> D[Check Convexity]
    ///     D --> E[Add to Space]
    ///     E --> F[Emit RegionAdded Event]
    ///     F --> G[Update Aggregate]
    /// ```
    fn handle(&mut self, envelope: CommandEnvelope<AddRegion>) -> CommandAcknowledgment {
        let command = envelope.command;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Add region (this validates convexity)
        match aggregate.add_region(command.region.clone()) {
            Ok(_) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
}

impl CommandHandler<ReplaceDimensionWeights> for ConceptualSpaceCommandHandler {
    /// Handle the ReplaceDimensionWeights command (DDD-compliant replacement)
    ///
    /// ```mermaid
    /// graph TD
    ///     A[ReplaceDimensionWeights] --> B[Load Aggregate]
    ///     B --> C[Get Current Weights]
    ///     C --> D[Emit WeightsRemoved Event]
    ///     D --> E[Emit WeightsAdded Event]
    ///     E --> F[Update Aggregate]
    /// ```
    fn handle(&mut self, envelope: CommandEnvelope<ReplaceDimensionWeights>) -> CommandAcknowledgment {
        let command = envelope.command;

        // Load aggregate
        let aggregate = match self.aggregates.get_mut(&command.space_id) {
            Some(agg) => agg,
            None => return CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some("Conceptual space not found".to_string()),
            }
        };

        // Get current weights for removal event
        let current_weights = aggregate.get_metric_weights();

        // Replace weights (this validates the weights)
        match aggregate.update_metric_weights(command.new_weights.clone()) {
            Ok(_) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Accepted,
                reason: None,
            },
            Err(e) => CommandAcknowledgment {
                command_id: envelope.id,
                correlation_id: envelope.correlation_id,
                status: CommandStatus::Rejected,
                reason: Some(e.to_string()),
            }
        }
    }
} 