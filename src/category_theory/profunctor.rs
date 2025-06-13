//! Profunctors for conceptual spaces

use crate::ConceptualResult;

/// A profunctor between conceptual spaces
pub trait ConceptualProfunctor {
    type Source;
    type Target;
    type Output;

    /// Apply the profunctor
    fn apply(&self, source: Self::Source, target: Self::Target) -> ConceptualResult<Self::Output>;
}

/// Bimap operation for profunctors
pub trait ProfunctorBimap: ConceptualProfunctor {
    /// Map over both arguments
    fn bimap<F, G>(&self, f: F, g: G) -> ConceptualResult<Self::Output>
    where
        F: Fn(Self::Source) -> Self::Source,
        G: Fn(Self::Target) -> Self::Target;
}
