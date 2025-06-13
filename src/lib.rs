//! Conceptual Spaces domain for CIM - Geometric knowledge representation
//!
//! This domain provides the foundation for representing all domain concepts
//! in a unified high-dimensional conceptual space where:
//! - Points represent individual concepts
//! - Regions represent categories
//! - Dimensions represent quality aspects
//! - Distance represents semantic similarity

// Domain modules following DDD pattern
pub mod aggregate;
pub mod value_objects;
pub mod commands;
pub mod events;
pub mod handlers;
pub mod queries;
pub mod projections;

// Original modules (domain logic)
pub mod space;
pub mod dimensions;
pub mod concept_map;
pub mod morphisms;
pub mod projection;
pub mod traits;
pub mod category_theory;

// Re-export aggregate
pub use aggregate::ConceptualSpaceAggregate;

// Re-export value objects
pub use value_objects::{
    Concept, QualityDimension, DimensionType, DimensionWeight,
    ConvexRegion, Hyperplane
};

// Re-export commands
pub use commands::{
    CreateConceptualSpace, AddConcept, AddRegion, UpdateDimensionWeights
};

// Re-export events
pub use events::{
    ConceptualSpaceDomainEvent, ConceptualSpaceCreated, ConceptAdded,
    RegionAdded, DimensionWeightsUpdated
};

// Re-export queries
pub use queries::{FindSimilarConcepts, SimilarConcepts};

// Re-export core types from original modules
pub use space::{
    ConceptualSpace, ConceptualPoint, ConceptualSpaceId, DimensionId,
    ConceptualMetric, OpenBall
};
pub use dimensions::{DistanceMetric, DimensionRegistry};
pub use concept_map::{ConceptMap, ConceptMapId, ConceptNode, ConceptEdge, ContextId};
pub use morphisms::{CrossContextMorphism, MorphismType, ConceptId};
pub use projection::{ConceptualProjection, ConceptualChange};
pub use traits::{ConceptualEntity, ConceptProducer};

use cim_domain::DomainError;
use thiserror::Error;

/// Errors that can occur in conceptual space operations
#[derive(Debug, Error)]
pub enum ConceptualError {
    /// Invalid dimension configuration
    #[error("Invalid dimension: {0}")]
    InvalidDimension(String),

    /// Point outside valid space
    #[error("Point outside valid space: {0}")]
    InvalidPoint(String),

    /// Morphism constraint violation
    #[error("Morphism constraint violation: {0}")]
    InvalidMorphism(String),

    /// Projection error
    #[error("Projection error: {0}")]
    ProjectionError(String),

    /// Domain error
    #[error("Domain error: {0}")]
    DomainError(#[from] DomainError),
}

/// Result type for conceptual operations
pub type ConceptualResult<T> = Result<T, ConceptualError>;
