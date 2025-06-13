//! Functors between conceptual spaces

use crate::concept_map::{ConceptMap, ContextId};
use crate::morphisms::CrossContextMorphism;
use crate::ConceptualResult;

/// A functor between conceptual spaces
pub trait ConceptualFunctor {
    /// Source context
    fn source_context(&self) -> ContextId;

    /// Target context
    fn target_context(&self) -> ContextId;

    /// Map a concept from source to target
    fn map_concept(&self, concept: &ConceptMap) -> ConceptualResult<ConceptMap>;

    /// Map a morphism from source to target
    fn map_morphism(&self, morphism: &CrossContextMorphism) -> ConceptualResult<CrossContextMorphism>;

    /// Check if the functor preserves composition
    fn preserves_composition(&self) -> bool {
        true // Most functors should preserve composition
    }

    /// Check if the functor preserves identity
    fn preserves_identity(&self) -> bool {
        true // Most functors should preserve identity
    }
}

/// A concrete functor mapping
pub struct FunctorMapping {
    pub source: ContextId,
    pub target: ContextId,
    pub concept_map: Box<dyn Fn(&ConceptMap) -> ConceptualResult<ConceptMap> + Send + Sync>,
    pub morphism_map: Box<dyn Fn(&CrossContextMorphism) -> ConceptualResult<CrossContextMorphism> + Send + Sync>,
}

// Since we can't derive Clone for function pointers, we'll implement a builder pattern
pub struct FunctorMappingBuilder {
    source: Option<ContextId>,
    target: Option<ContextId>,
}

impl FunctorMappingBuilder {
    pub fn new() -> Self {
        Self {
            source: None,
            target: None,
        }
    }

    pub fn from_context(mut self, context: ContextId) -> Self {
        self.source = Some(context);
        self
    }

    pub fn to_context(mut self, context: ContextId) -> Self {
        self.target = Some(context);
        self
    }

    pub fn build<F, G>(self, concept_fn: F, morphism_fn: G) -> FunctorMapping
    where
        F: Fn(&ConceptMap) -> ConceptualResult<ConceptMap> + Send + Sync + 'static,
        G: Fn(&CrossContextMorphism) -> ConceptualResult<CrossContextMorphism> + Send + Sync + 'static,
    {
        FunctorMapping {
            source: self.source.expect("Source context required"),
            target: self.target.expect("Target context required"),
            concept_map: Box::new(concept_fn),
            morphism_map: Box::new(morphism_fn),
        }
    }
}

impl Default for FunctorMappingBuilder {
    fn default() -> Self {
        Self::new()
    }
}
