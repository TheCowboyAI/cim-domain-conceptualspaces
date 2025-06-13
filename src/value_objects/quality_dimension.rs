//! Quality dimension value object

use crate::{DimensionId, ConceptualError, ConceptualResult};
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
