//! Event to conceptual space projection

use crate::morphisms::ConceptId;
use crate::space::{ConceptualPoint, DimensionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid;

/// Changes that can occur in conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptualChange {
    /// Add a new concept to the space
    AddConcept {
        concept_id: ConceptId,
        concept_type: String,
        position: ConceptualPoint,
        qualities: HashMap<DimensionId, f64>,
    },

    /// Remove a concept from the space
    RemoveConcept {
        concept_id: ConceptId,
    },

    /// Add a concept to a region
    AddToRegion {
        concept_id: ConceptId,
        region_id: uuid::Uuid,
    },

    /// Remove a concept from a region
    RemoveFromRegion {
        concept_id: ConceptId,
        region_id: uuid::Uuid,
    },
}

/// Trait for events that can be projected into conceptual space
pub trait ConceptualProjection {
    /// Project this event into conceptual changes
    fn project(&self) -> Vec<ConceptualChange>;

    /// Determine which concepts this event affects
    fn affected_concepts(&self) -> Vec<ConceptId>;

    /// Calculate quality dimension values for new concepts
    fn concept_qualities(&self) -> HashMap<DimensionId, f64>;

    /// Get the event type for categorization
    fn event_type(&self) -> &str;
}

/// Projection context containing dimension mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectionContext {
    /// Maps event properties to quality dimensions
    pub property_to_dimension: HashMap<String, DimensionId>,

    /// Default quality values for new concepts
    pub default_qualities: HashMap<DimensionId, f64>,

    /// Transformation functions for property values
    pub transformations: HashMap<String, TransformationType>,
}

/// Types of transformations for property values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformationType {
    /// Direct mapping (no transformation)
    Identity,

    /// Linear scaling
    Linear { scale: f64, offset: f64 },

    /// Logarithmic transformation
    Logarithmic { base: f64 },

    /// Sigmoid transformation (for bounded values)
    Sigmoid { steepness: f64, midpoint: f64 },

    /// Custom transformation name
    Custom(String),
}

impl TransformationType {
    /// Apply the transformation to a value
    pub fn transform(&self, value: f64) -> f64 {
        match self {
            TransformationType::Identity => value,
            TransformationType::Linear { scale, offset } => value * scale + offset,
            TransformationType::Logarithmic { base } => value.log(*base),
            TransformationType::Sigmoid { steepness, midpoint } => {
                1.0 / (1.0 + (-steepness * (value - midpoint)).exp())
            }
            TransformationType::Custom(_) => value, // Custom transformations handled elsewhere
        }
    }
}

/// Builder for creating projection contexts
pub struct ProjectionContextBuilder {
    context: ProjectionContext,
}

impl ProjectionContextBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            context: ProjectionContext {
                property_to_dimension: HashMap::new(),
                default_qualities: HashMap::new(),
                transformations: HashMap::new(),
            },
        }
    }

    /// Map a property to a dimension
    pub fn map_property(mut self, property: String, dimension: DimensionId) -> Self {
        self.context.property_to_dimension.insert(property, dimension);
        self
    }

    /// Set a default quality value
    pub fn with_default(mut self, dimension: DimensionId, value: f64) -> Self {
        self.context.default_qualities.insert(dimension, value);
        self
    }

    /// Add a transformation for a property
    pub fn with_transformation(mut self, property: String, transformation: TransformationType) -> Self {
        self.context.transformations.insert(property, transformation);
        self
    }

    /// Build the projection context
    pub fn build(self) -> ProjectionContext {
        self.context
    }
}

impl Default for ProjectionContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Example implementation for a generic domain event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleDomainEvent {
    pub entity_id: String,
    pub event_type: String,
    pub properties: HashMap<String, f64>,
}

impl ConceptualProjection for ExampleDomainEvent {
    fn project(&self) -> Vec<ConceptualChange> {
        // In event-driven architecture, changes are represented as remove + add
        // This is a simplified example - real implementations would be more sophisticated
        let concept_id = ConceptId::new(); // Would map from entity_id

        vec![
            // First remove the old concept
            ConceptualChange::RemoveConcept { concept_id },
            // Then add the new concept with updated qualities
            ConceptualChange::AddConcept {
                concept_id,
                concept_type: self.event_type.clone(),
                position: ConceptualPoint::new(vec![], HashMap::new()), // Would calculate position
                qualities: self.concept_qualities(),
            }
        ]
    }

    fn affected_concepts(&self) -> Vec<ConceptId> {
        vec![ConceptId::new()] // Would map from entity_id
    }

    fn concept_qualities(&self) -> HashMap<DimensionId, f64> {
        // Would use ProjectionContext to map properties to dimensions
        HashMap::new()
    }

    fn event_type(&self) -> &str {
        &self.event_type
    }
}
