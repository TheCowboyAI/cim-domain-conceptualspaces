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
                // For zero-range dimensions, only the exact value is valid
                if self.range.start == self.range.end {
                    if value == self.range.start {
                        Ok(())
                    } else {
                        Err(ConceptualError::InvalidDimension(
                            format!("Value {} must equal {} for zero-range dimension '{}'",
                                    value, self.range.start, self.name)
                        ))
                    }
                } else if value >= self.range.start && value < self.range.end {
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
        if !(0.0..=1.0).contains(&normalized) {
            return Err(ConceptualError::InvalidDimension(
                format!("Normalized value {normalized} must be in [0, 1]")
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test continuous dimension creation and validation
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Continuous] --> B[Validate In Range]
    ///     B --> C[Reject Out of Range]
    ///     C --> D[Normalize Values]
    /// ```
    #[test]
    fn test_continuous_dimension() {
        let dim = QualityDimension::continuous("Temperature".to_string(), -10.0, 40.0);

        // Test valid values
        assert!(dim.validate_value(0.0).is_ok());
        assert!(dim.validate_value(-10.0).is_ok());
        assert!(dim.validate_value(39.9).is_ok());

        // Test invalid values
        assert!(dim.validate_value(-10.1).is_err());
        assert!(dim.validate_value(40.0).is_err());
        assert!(dim.validate_value(100.0).is_err());

        // Test normalization
        assert_eq!(dim.normalize_value(-10.0).unwrap(), 0.0);
        assert_eq!(dim.normalize_value(15.0).unwrap(), 0.5);
        assert_eq!(dim.normalize_value(39.9).unwrap(), 0.998);

        // Test denormalization
        assert_eq!(dim.denormalize_value(0.0).unwrap(), -10.0);
        assert_eq!(dim.denormalize_value(0.5).unwrap(), 15.0);
        assert_eq!(dim.denormalize_value(1.0).unwrap(), 40.0);
    }

    /// Test categorical dimension
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Categorical] --> B[N Categories]
    ///     B --> C[Validate Category Index]
    ///     C --> D[Normalize to [0,1]]
    /// ```
    #[test]
    fn test_categorical_dimension() {
        let dim = QualityDimension::categorical("Color".to_string(), 5);

        // Test valid categories (0-4)
        assert!(dim.validate_value(0.0).is_ok());
        assert!(dim.validate_value(1.0).is_ok());
        assert!(dim.validate_value(4.9).is_ok());

        // Test invalid categories
        assert!(dim.validate_value(-1.0).is_err());
        assert!(dim.validate_value(5.0).is_err());

        // Test normalization
        assert_eq!(dim.normalize_value(0.0).unwrap(), 0.0);
        assert_eq!(dim.normalize_value(2.5).unwrap(), 0.5);
        assert_eq!(dim.normalize_value(4.0).unwrap(), 0.8);
    }

    /// Test ordinal dimension
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Ordinal] --> B[Ordered Levels]
    ///     B --> C[Validate Level]
    ///     C --> D[Preserve Order]
    /// ```
    #[test]
    fn test_ordinal_dimension() {
        let dim = QualityDimension::ordinal("Size".to_string(), 3); // Small, Medium, Large

        // Test valid levels (0, 1, 2)
        assert!(dim.validate_value(0.0).is_ok());
        assert!(dim.validate_value(1.0).is_ok());
        assert!(dim.validate_value(2.9).is_ok());

        // Test invalid levels
        assert!(dim.validate_value(-1.0).is_err());
        assert!(dim.validate_value(3.0).is_err());

        // Test that ordinal preserves order in normalization
        let norm_small = dim.normalize_value(0.0).unwrap();
        let norm_medium = dim.normalize_value(1.0).unwrap();
        let norm_large = dim.normalize_value(2.0).unwrap();

        assert!(norm_small < norm_medium);
        assert!(norm_medium < norm_large);
    }

    /// Test circular dimension
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Circular] --> B[0-360 Degrees]
    ///     B --> C[Wraps Around]
    ///     C --> D[Always Valid]
    /// ```
    #[test]
    fn test_circular_dimension() {
        let dim = QualityDimension::circular("Direction".to_string());

        // All values should be valid (wrapping)
        assert!(dim.validate_value(0.0).is_ok());
        assert!(dim.validate_value(180.0).is_ok());
        assert!(dim.validate_value(359.9).is_ok());
        assert!(dim.validate_value(720.0).is_ok()); // Wraps around
        assert!(dim.validate_value(-90.0).is_ok()); // Wraps around

        // Test normalization with wrapping
        assert_eq!(dim.normalize_value(0.0).unwrap(), 0.0);
        assert_eq!(dim.normalize_value(180.0).unwrap(), 0.5);
        assert_eq!(dim.normalize_value(360.0).unwrap(), 0.0); // Wraps to 0
        assert_eq!(dim.normalize_value(720.0).unwrap(), 0.0); // 720 % 360 = 0

        // Test denormalization
        assert_eq!(dim.denormalize_value(0.0).unwrap(), 0.0);
        assert_eq!(dim.denormalize_value(0.5).unwrap(), 180.0);
        assert_eq!(dim.denormalize_value(1.0).unwrap(), 360.0);
    }

    /// Test dimension with context and description
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Dimension] --> B[Add Context]
    ///     B --> C[Add Description]
    ///     C --> D[Verify Metadata]
    /// ```
    #[test]
    fn test_dimension_metadata() {
        let mut dim = QualityDimension::continuous("Price".to_string(), 0.0, 1000.0);
        dim.context = Some("E-commerce".to_string());
        dim.description = Some("Product price in USD".to_string());

        assert_eq!(dim.context.as_ref().unwrap(), "E-commerce");
        assert_eq!(dim.description.as_ref().unwrap(), "Product price in USD");
    }

    /// Test edge cases in normalization
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Edge Cases] --> B[Zero Range]
    ///     B --> C[Invalid Normalized]
    ///     C --> D[Boundary Values]
    /// ```
    #[test]
    fn test_normalization_edge_cases() {
        // Zero-range dimension
        let zero_range = QualityDimension::continuous("Constant".to_string(), 5.0, 5.0);
        assert_eq!(zero_range.normalize_value(5.0).unwrap(), 0.0);

        // Invalid normalized values
        let dim = QualityDimension::continuous("Test".to_string(), 0.0, 10.0);
        assert!(dim.denormalize_value(-0.1).is_err());
        assert!(dim.denormalize_value(1.1).is_err());

        // Boundary values
        assert_eq!(dim.denormalize_value(0.0).unwrap(), 0.0);
        assert_eq!(dim.denormalize_value(1.0).unwrap(), 10.0);
    }
}
