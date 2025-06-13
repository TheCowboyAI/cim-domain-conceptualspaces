//! Applied Category Theory structures for conceptual spaces
//!
//! This module provides category-theoretic foundations for understanding
//! and manipulating conceptual spaces, including functors, profunctors,
//! and operads.

pub mod functor;
pub mod profunctor;
pub mod operad;

// Re-export main types
pub use functor::{ConceptualFunctor, FunctorMapping};
pub use profunctor::{ConceptualProfunctor, ProfunctorBimap};
pub use operad::{ConceptualOperad, Operation};
