//! Conceptual space types and traits for unified knowledge representation
//!
//! This module provides the foundation for representing all domain concepts
//! in a unified high-dimensional conceptual space where:
//! - Points represent individual concepts
//! - Regions represent categories
//! - Dimensions represent quality aspects
//! - Distance represents semantic similarity

pub mod space;
pub mod dimensions;
pub mod concept_map;
pub mod morphisms;
pub mod projection;
pub mod traits;
pub mod category_theory;

// Re-export core types
pub use space::{
    ConceptualSpace, ConceptualPoint, ConvexRegion, ConceptualSpaceId, DimensionId,
    DimensionWeight, Hyperplane, ConceptualMetric, OpenBall
};
pub use dimensions::{QualityDimension, DimensionType, DistanceMetric, DimensionRegistry};
pub use concept_map::{ConceptMap, ConceptMapId, ConceptNode, ConceptEdge, ContextId};
pub use morphisms::{CrossContextMorphism, MorphismType, ConceptId};
pub use projection::{ConceptualProjection, ConceptualChange};
pub use traits::{ConceptualEntity, ConceptProducer};

use cim_core_domain::DomainError;
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
