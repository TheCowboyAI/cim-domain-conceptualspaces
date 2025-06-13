//! Core traits for conceptual space integration

use crate::concept_map::{ConceptMap, ContextId};
use crate::dimensions::QualityDimension;
use crate::space::{ConceptualPoint, DimensionId};
use crate::ConceptualResult;
use std::collections::HashMap;

/// Trait for entities that exist in conceptual space
pub trait ConceptualEntity {
    /// Get the concept's position in conceptual space
    fn conceptual_position(&self) -> ConceptualPoint;

    /// Get the concept's quality values
    fn qualities(&self) -> HashMap<DimensionId, f64>;

    /// Convert to a ConceptMap for storage
    fn to_concept_map(&self) -> ConceptMap;

    /// Get the entity's unique identifier
    fn entity_id(&self) -> uuid::Uuid;

    /// Get the entity's concept type
    fn concept_type(&self) -> &str;
}

/// Trait for bounded contexts that produce concepts
pub trait ConceptProducer {
    /// The type of concepts this context produces
    type Concept: ConceptualEntity;

    /// The type of events this context processes
    type Event;

    /// Produce concepts from a domain event
    fn produce_concepts(&self, event: Self::Event) -> Vec<Self::Concept>;

    /// Get the quality dimensions this context contributes
    fn concept_dimensions(&self) -> Vec<QualityDimension>;

    /// Get the context identifier
    fn context_id(&self) -> ContextId;

    /// Initialize the context's conceptual space
    fn initialize_space(&self) -> ConceptualResult<()>;
}

/// Trait for concept storage and retrieval
#[async_trait::async_trait]
pub trait ConceptStore {
    /// Store a concept map and return its CID
    async fn store_concept(&self, concept: ConceptMap) -> ConceptualResult<cid::Cid>;

    /// Retrieve a concept map by CID
    async fn get_concept(&self, cid: &cid::Cid) -> ConceptualResult<ConceptMap>;

    /// Find concepts in a region of conceptual space
    async fn find_concepts_in_region(
        &self,
        center: ConceptualPoint,
        radius: f64,
    ) -> ConceptualResult<Vec<ConceptMap>>;

    /// Find concepts by quality criteria
    async fn find_by_qualities(
        &self,
        criteria: QualityCriteria,
    ) -> ConceptualResult<Vec<ConceptMap>>;

    /// Update a concept map
    async fn update_concept(&self, concept: ConceptMap) -> ConceptualResult<cid::Cid>;

    /// Delete a concept by CID
    async fn delete_concept(&self, cid: &cid::Cid) -> ConceptualResult<()>;
}

/// Criteria for quality-based searches
#[derive(Debug, Clone)]
pub struct QualityCriteria {
    /// Required quality values (must match exactly)
    pub required: HashMap<DimensionId, f64>,

    /// Minimum quality values
    pub minimum: HashMap<DimensionId, f64>,

    /// Maximum quality values
    pub maximum: HashMap<DimensionId, f64>,

    /// Quality values that must be present (any value)
    pub must_have: Vec<DimensionId>,
}

impl QualityCriteria {
    /// Create empty criteria
    pub fn new() -> Self {
        Self {
            required: HashMap::new(),
            minimum: HashMap::new(),
            maximum: HashMap::new(),
            must_have: Vec::new(),
        }
    }

    /// Add a required quality value
    pub fn with_required(mut self, dimension: DimensionId, value: f64) -> Self {
        self.required.insert(dimension, value);
        self
    }

    /// Add a minimum quality value
    pub fn with_minimum(mut self, dimension: DimensionId, value: f64) -> Self {
        self.minimum.insert(dimension, value);
        self
    }

    /// Add a maximum quality value
    pub fn with_maximum(mut self, dimension: DimensionId, value: f64) -> Self {
        self.maximum.insert(dimension, value);
        self
    }

    /// Add a dimension that must be present
    pub fn must_have_dimension(mut self, dimension: DimensionId) -> Self {
        self.must_have.push(dimension);
        self
    }

    /// Check if a concept map matches these criteria
    pub fn matches(&self, concept: &ConceptMap) -> bool {
        // Check required values
        for (dim, value) in &self.required {
            match concept.qualities.get(dim) {
                Some(v) if (*v - value).abs() < f64::EPSILON => continue,
                _ => return false,
            }
        }

        // Check minimum values
        for (dim, min) in &self.minimum {
            match concept.qualities.get(dim) {
                Some(v) if *v >= *min => continue,
                _ => return false,
            }
        }

        // Check maximum values
        for (dim, max) in &self.maximum {
            match concept.qualities.get(dim) {
                Some(v) if *v <= *max => continue,
                _ => return false,
            }
        }

        // Check must-have dimensions
        for dim in &self.must_have {
            if !concept.qualities.contains_key(dim) {
                return false;
            }
        }

        true
    }
}

impl Default for QualityCriteria {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for morphism discovery between concepts
pub trait MorphismDiscoverer {
    /// Discover morphisms between two concept maps
    fn discover_morphisms(
        &self,
        concept_a: &ConceptMap,
        concept_b: &ConceptMap,
    ) -> Vec<crate::morphisms::CrossContextMorphism>;

    /// Set the similarity threshold for morphism discovery
    fn set_threshold(&mut self, threshold: f64);

    /// Get the current similarity threshold
    fn threshold(&self) -> f64;
}
