//! Concept value object representing a point in conceptual space

use crate::DimensionId;
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A concept represented as a point in the conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    /// The n-dimensional coordinates
    pub coordinates: DVector<f64>,

    /// Maps dimension IDs to coordinate indices
    pub dimension_map: HashMap<DimensionId, usize>,

    /// Optional concept identifier
    pub id: Option<Uuid>,

    /// Optional name for the concept
    pub name: Option<String>,

    /// Optional description
    pub description: Option<String>,
}

impl Concept {
    /// Create a new concept
    pub fn new(coordinates: Vec<f64>, dimension_map: HashMap<DimensionId, usize>) -> Self {
        Self {
            coordinates: DVector::from_vec(coordinates),
            dimension_map,
            id: Some(Uuid::new_v4()),
            name: None,
            description: None,
        }
    }

    /// Create a named concept
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Create a concept with description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Get the value for a specific dimension
    pub fn get_dimension_value(&self, dimension_id: &DimensionId) -> Option<f64> {
        self.dimension_map
            .get(dimension_id)
            .and_then(|&idx| self.coordinates.get(idx).copied())
    }

    /// Calculate weighted Minkowski distance to another concept
    pub fn weighted_distance(
        &self,
        other: &Concept,
        weights: &[f64],
        p: f64,
    ) -> Result<f64, String> {
        if self.coordinates.len() != other.coordinates.len() {
            return Err("Concepts have different dimensions".to_string());
        }

        if weights.len() != self.coordinates.len() {
            return Err("Weight vector has incorrect length".to_string());
        }

        // Minkowski distance with weights
        let sum: f64 = self.coordinates.iter()
            .zip(other.coordinates.iter())
            .zip(weights.iter())
            .map(|((a, b), w)| w * (a - b).abs().powf(p))
            .sum();

        Ok(sum.powf(1.0 / p))
    }
}
