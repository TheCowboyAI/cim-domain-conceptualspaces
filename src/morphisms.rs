//! Cross-context morphisms and relationships

use crate::concept_map::ContextId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for a concept
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConceptId(Uuid);

impl ConceptId {
    /// Create a new random concept ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from an existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for ConceptId {
    fn default() -> Self {
        Self::new()
    }
}

/// Types of morphisms between concepts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MorphismType {
    /// Identity Person ↔ Security User mapping
    IdentityMapping,

    /// Security Policy → Workflow Permissions
    PolicyApplication,

    /// Workflow State → Content Status
    StateMapping,

    /// Knowledge Concept ↔ All Contexts (semantic similarity)
    SemanticLink,

    /// Hierarchical relationship (parent-child)
    Hierarchy { parent_role: String, child_role: String },

    /// Temporal relationship (before-after)
    Temporal { relationship: String },

    /// Causal relationship
    Causal { cause_role: String, effect_role: String },

    /// Custom morphism type
    Custom(String),
}

/// A morphism between concepts across contexts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossContextMorphism {
    /// Unique identifier for this morphism
    pub id: Uuid,

    /// Source concept
    pub source: (ContextId, ConceptId),

    /// Target concept
    pub target: (ContextId, ConceptId),

    /// Type of morphism
    pub morphism_type: MorphismType,

    /// Strength of the relationship (0.0 to 1.0)
    pub strength: f64,

    /// Whether this morphism is bidirectional
    pub bidirectional: bool,

    /// Additional metadata
    pub metadata: serde_json::Value,
}

impl CrossContextMorphism {
    /// Create a new morphism
    pub fn new(
        source: (ContextId, ConceptId),
        target: (ContextId, ConceptId),
        morphism_type: MorphismType,
        strength: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            target,
            morphism_type,
            strength,
            bidirectional: false,
            metadata: serde_json::Value::Null,
        }
    }

    /// Create a bidirectional morphism
    pub fn bidirectional(
        source: (ContextId, ConceptId),
        target: (ContextId, ConceptId),
        morphism_type: MorphismType,
        strength: f64,
    ) -> Self {
        let mut morphism = Self::new(source, target, morphism_type, strength);
        morphism.bidirectional = true;
        morphism
    }

    /// Check if this morphism connects two specific contexts
    pub fn connects_contexts(&self, context1: ContextId, context2: ContextId) -> bool {
        (self.source.0 == context1 && self.target.0 == context2) ||
        (self.source.0 == context2 && self.target.0 == context1)
    }

    /// Check if this morphism involves a specific concept
    pub fn involves_concept(&self, concept_id: ConceptId) -> bool {
        self.source.1 == concept_id || self.target.1 == concept_id
    }

    /// Get the inverse morphism (if bidirectional)
    pub fn inverse(&self) -> Option<Self> {
        if self.bidirectional {
            Some(Self {
                id: Uuid::new_v4(),
                source: self.target,
                target: self.source,
                morphism_type: self.morphism_type.clone(),
                strength: self.strength,
                bidirectional: true,
                metadata: self.metadata.clone(),
            })
        } else {
            None
        }
    }
}

/// Discovery rules for finding morphisms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphismDiscoveryRule {
    /// Name of the rule
    pub name: String,

    /// Source context pattern
    pub source_context: Option<ContextId>,

    /// Target context pattern
    pub target_context: Option<ContextId>,

    /// Minimum similarity threshold
    pub similarity_threshold: f64,

    /// Type of morphism to create
    pub morphism_type: MorphismType,

    /// Whether to create bidirectional morphisms
    pub bidirectional: bool,
}

impl MorphismDiscoveryRule {
    /// Create a new discovery rule
    pub fn new(name: String, morphism_type: MorphismType) -> Self {
        Self {
            name,
            source_context: None,
            target_context: None,
            similarity_threshold: 0.7,
            morphism_type,
            bidirectional: false,
        }
    }

    /// Set source context filter
    pub fn with_source_context(mut self, context: ContextId) -> Self {
        self.source_context = Some(context);
        self
    }

    /// Set target context filter
    pub fn with_target_context(mut self, context: ContextId) -> Self {
        self.target_context = Some(context);
        self
    }

    /// Set similarity threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold;
        self
    }

    /// Make bidirectional
    pub fn bidirectional(mut self) -> Self {
        self.bidirectional = true;
        self
    }

    /// Check if this rule applies to a pair of contexts
    pub fn applies_to(&self, source: ContextId, target: ContextId) -> bool {
        let source_match = self.source_context.map_or(true, |c| c == source);
        let target_match = self.target_context.map_or(true, |c| c == target);
        source_match && target_match
    }
}

/// Collection of morphisms with query capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MorphismCollection {
    /// All morphisms indexed by ID
    morphisms: Vec<CrossContextMorphism>,
}

impl MorphismCollection {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a morphism to the collection
    pub fn add(&mut self, morphism: CrossContextMorphism) {
        self.morphisms.push(morphism);
    }

    /// Find all morphisms for a specific concept
    pub fn find_by_concept(&self, concept_id: ConceptId) -> Vec<&CrossContextMorphism> {
        self.morphisms.iter()
            .filter(|m| m.involves_concept(concept_id))
            .collect()
    }

    /// Find all morphisms between two contexts
    pub fn find_between_contexts(
        &self,
        context1: ContextId,
        context2: ContextId,
    ) -> Vec<&CrossContextMorphism> {
        self.morphisms.iter()
            .filter(|m| m.connects_contexts(context1, context2))
            .collect()
    }

    /// Find all morphisms of a specific type
    pub fn find_by_type(&self, morphism_type: &MorphismType) -> Vec<&CrossContextMorphism> {
        self.morphisms.iter()
            .filter(|m| &m.morphism_type == morphism_type)
            .collect()
    }

    /// Find the strongest morphism between two concepts
    pub fn find_strongest(
        &self,
        source: ConceptId,
        target: ConceptId,
    ) -> Option<&CrossContextMorphism> {
        self.morphisms.iter()
            .filter(|m| {
                (m.source.1 == source && m.target.1 == target) ||
                (m.bidirectional && m.source.1 == target && m.target.1 == source)
            })
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
    }

    /// Get all morphisms
    pub fn all(&self) -> &[CrossContextMorphism] {
        &self.morphisms
    }

    /// Get the total number of morphisms
    pub fn len(&self) -> usize {
        self.morphisms.len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.morphisms.is_empty()
    }
}
