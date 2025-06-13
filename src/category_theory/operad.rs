//! Operads for composing operations in conceptual spaces

use crate::ConceptualResult;

/// An operad for conceptual space operations
pub trait ConceptualOperad {
    type Input;
    type Output;

    /// Compose operations
    fn compose(&self, operations: Vec<Operation<Self::Input, Self::Output>>) -> ConceptualResult<Operation<Self::Input, Self::Output>>;
}

/// An operation in the operad
pub struct Operation<I, O> {
    pub arity: usize,
    pub apply: Box<dyn Fn(Vec<I>) -> ConceptualResult<O> + Send + Sync>,
}

impl<I, O> Operation<I, O> {
    /// Create a new operation
    pub fn new<F>(arity: usize, f: F) -> Self
    where
        F: Fn(Vec<I>) -> ConceptualResult<O> + Send + Sync + 'static,
    {
        Self {
            arity,
            apply: Box::new(f),
        }
    }
}
