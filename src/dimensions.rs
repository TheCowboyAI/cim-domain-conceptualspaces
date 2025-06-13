//! Quality dimensions and distance metrics

use crate::space::{ConceptualPoint, DimensionId};
use crate::{ConceptualError, ConceptualResult};
use serde::{Deserialize, Serialize};
use std::ops::Range;

/// Type of quality dimension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DimensionType {
    /// Continuous values (e.g., temperature, size)
    Continuous,
    /// Categorical values (e.g., color categories)
    Categorical,
    /// Ordinal values (e.g., small < medium < large)
    Ordinal,
    /// Circular values (e.g., hue, direction in degrees)
    Circular,
}

/// Distance metric for measuring similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistanceMetric {
    /// Standard Euclidean distance
    Euclidean,
    /// Manhattan (city block) distance
    Manhattan,
    /// Weighted Euclidean with dimension weights
    WeightedEuclidean { weights: Vec<f64> },
    /// Cosine similarity (angle between vectors)
    Cosine,
    /// Custom metric with a name
    Custom(String),
}

impl DistanceMetric {
    /// Calculate distance between two points using this metric
    pub fn calculate(&self, a: &ConceptualPoint, b: &ConceptualPoint) -> ConceptualResult<f64> {
        if a.coordinates.len() != b.coordinates.len() {
            return Err(ConceptualError::InvalidPoint(
                "Points have different dimensions".to_string()
            ));
        }

        match self {
            DistanceMetric::Euclidean => {
                Ok((&a.coordinates - &b.coordinates).norm())
            }
            DistanceMetric::Manhattan => {
                let sum: f64 = a.coordinates.iter()
                    .zip(b.coordinates.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum();
                Ok(sum)
            }
            DistanceMetric::WeightedEuclidean { weights } => {
                if weights.len() != a.coordinates.len() {
                    return Err(ConceptualError::InvalidDimension(
                        "Weight vector has incorrect length".to_string()
                    ));
                }

                let sum: f64 = a.coordinates.iter()
                    .zip(b.coordinates.iter())
                    .zip(weights.iter())
                    .map(|((a, b), w)| w * (a - b).powi(2))
                    .sum();
                Ok(sum.sqrt())
            }
            DistanceMetric::Cosine => {
                let dot_product: f64 = a.coordinates.dot(&b.coordinates);
                let norm_a = a.coordinates.norm();
                let norm_b = b.coordinates.norm();

                if norm_a == 0.0 || norm_b == 0.0 {
                    return Err(ConceptualError::InvalidPoint(
                        "Cannot calculate cosine distance for zero vector".to_string()
                    ));
                }

                let cosine_similarity = dot_product / (norm_a * norm_b);
                // Convert to distance (0 = identical, 2 = opposite)
                Ok(1.0 - cosine_similarity)
            }
            DistanceMetric::Custom(name) => {
                Err(ConceptualError::InvalidDimension(
                    format!("Custom metric '{}' not implemented", name)
                ))
            }
        }
    }
}

/// A quality dimension in conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDimension {
    /// Unique identifier for this dimension
    pub id: DimensionId,

    /// Human-readable name
    pub name: String,

    /// Type of dimension
    pub dimension_type: DimensionType,

    /// Valid range of values
    pub range: Range<f64>,

    /// Distance metric for this dimension
    pub metric: DistanceMetric,

    /// Optional context this dimension belongs to
    pub context: Option<String>,

    /// Description of what this dimension represents
    pub description: Option<String>,
}

impl QualityDimension {
    /// Create a new quality dimension
    pub fn new(name: String, dimension_type: DimensionType, range: Range<f64>) -> Self {
        Self {
            id: DimensionId::new(),
            name,
            dimension_type,
            range,
            metric: DistanceMetric::Euclidean,
            context: None,
            description: None,
        }
    }

    /// Create a continuous dimension
    pub fn continuous(name: String, min: f64, max: f64) -> Self {
        Self::new(name, DimensionType::Continuous, min..max)
    }

    /// Create a categorical dimension
    pub fn categorical(name: String, num_categories: usize) -> Self {
        Self::new(name, DimensionType::Categorical, 0.0..(num_categories as f64))
    }

    /// Create an ordinal dimension
    pub fn ordinal(name: String, num_levels: usize) -> Self {
        Self::new(name, DimensionType::Ordinal, 0.0..(num_levels as f64))
    }

    /// Create a circular dimension (0-360 degrees)
    pub fn circular(name: String) -> Self {
        Self::new(name, DimensionType::Circular, 0.0..360.0)
    }

    /// Validate that a value is within the dimension's range
    pub fn validate_value(&self, value: f64) -> ConceptualResult<()> {
        match self.dimension_type {
            DimensionType::Circular => {
                // Circular dimensions wrap around
                Ok(())
            }
            _ => {
                if value >= self.range.start && value < self.range.end {
                    Ok(())
                } else {
                    Err(ConceptualError::InvalidDimension(
                        format!("Value {} is outside range {:?} for dimension '{}'",
                                value, self.range, self.name)
                    ))
                }
            }
        }
    }

    /// Normalize a value to [0, 1] range
    pub fn normalize_value(&self, value: f64) -> ConceptualResult<f64> {
        self.validate_value(value)?;

        match self.dimension_type {
            DimensionType::Circular => {
                // Normalize circular values to [0, 1]
                Ok((value % 360.0) / 360.0)
            }
            _ => {
                let range_size = self.range.end - self.range.start;
                if range_size == 0.0 {
                    Ok(0.0)
                } else {
                    Ok((value - self.range.start) / range_size)
                }
            }
        }
    }

    /// Denormalize a value from [0, 1] to the dimension's range
    pub fn denormalize_value(&self, normalized: f64) -> ConceptualResult<f64> {
        if normalized < 0.0 || normalized > 1.0 {
            return Err(ConceptualError::InvalidDimension(
                format!("Normalized value {} must be in [0, 1]", normalized)
            ));
        }

        match self.dimension_type {
            DimensionType::Circular => {
                Ok(normalized * 360.0)
            }
            _ => {
                let range_size = self.range.end - self.range.start;
                Ok(self.range.start + normalized * range_size)
            }
        }
    }
}

/// Registry of quality dimensions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DimensionRegistry {
    /// All registered dimensions
    dimensions: Vec<QualityDimension>,
}

impl DimensionRegistry {
    /// Create a new dimension registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new dimension
    pub fn register(&mut self, dimension: QualityDimension) -> ConceptualResult<()> {
        // Check for duplicate names
        if self.dimensions.iter().any(|d| d.name == dimension.name) {
            return Err(ConceptualError::InvalidDimension(
                format!("Dimension '{}' already registered", dimension.name)
            ));
        }

        self.dimensions.push(dimension);
        Ok(())
    }

    /// Get a dimension by ID
    pub fn get(&self, id: &DimensionId) -> Option<&QualityDimension> {
        self.dimensions.iter().find(|d| d.id == *id)
    }

    /// Get a dimension by name
    pub fn get_by_name(&self, name: &str) -> Option<&QualityDimension> {
        self.dimensions.iter().find(|d| d.name == name)
    }

    /// Get all dimensions for a specific context
    pub fn get_by_context(&self, context: &str) -> Vec<&QualityDimension> {
        self.dimensions.iter()
            .filter(|d| d.context.as_deref() == Some(context))
            .collect()
    }

    /// Get all dimensions
    pub fn all(&self) -> &[QualityDimension] {
        &self.dimensions
    }
}
