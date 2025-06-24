//! Dimension weight value object for metric calculations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Weight function for dimensions in the metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionWeight {
    /// Constant weight
    Constant(f64),

    /// Context-dependent weight
    Contextual {
        base_weight: f64,
        context_modifiers: HashMap<String, f64>,
    },

    /// Attention-based weight (changes based on focus)
    Attentional {
        current_weight: f64,
        min_weight: f64,
        max_weight: f64,
    },
}

impl DimensionWeight {
    /// Get the current weight value
    pub fn value(&self, context: Option<&str>) -> f64 {
        match self {
            DimensionWeight::Constant(w) => *w,
            DimensionWeight::Contextual { base_weight, context_modifiers } => {
                context.and_then(|ctx| context_modifiers.get(ctx))
                    .copied()
                    .unwrap_or(*base_weight)
            }
            DimensionWeight::Attentional { current_weight, .. } => *current_weight,
        }
    }

    /// Create a constant weight
    pub fn constant(weight: f64) -> Self {
        DimensionWeight::Constant(weight)
    }

    /// Create a contextual weight
    pub fn contextual(base_weight: f64) -> Self {
        DimensionWeight::Contextual {
            base_weight,
            context_modifiers: HashMap::new(),
        }
    }

    /// Add a context modifier to a contextual weight
    pub fn with_context(mut self, context: String, weight: f64) -> Self {
        if let DimensionWeight::Contextual { ref mut context_modifiers, .. } = self {
            context_modifiers.insert(context, weight);
        }
        self
    }

    /// Create an attentional weight
    pub fn attentional(current: f64, min: f64, max: f64) -> Self {
        DimensionWeight::Attentional {
            current_weight: current.clamp(min, max),
            min_weight: min,
            max_weight: max,
        }
    }

    /// Update attentional weight
    pub fn update_attention(&mut self, new_weight: f64) {
        if let DimensionWeight::Attentional { current_weight, min_weight, max_weight } = self {
            *current_weight = new_weight.clamp(*min_weight, *max_weight);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test constant weight
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Constant] --> B[Always Same Value]
    ///     B --> C[Context Ignored]
    /// ```
    #[test]
    fn test_constant_weight() {
        let weight = DimensionWeight::constant(0.7);

        // Should always return the same value
        assert_eq!(weight.value(None), 0.7);
        assert_eq!(weight.value(Some("context1")), 0.7);
        assert_eq!(weight.value(Some("context2")), 0.7);
    }

    /// Test contextual weight
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Contextual] --> B[Base Weight]
    ///     B --> C[Add Context Modifiers]
    ///     C --> D[Context-Specific Values]
    /// ```
    #[test]
    fn test_contextual_weight() {
        let weight = DimensionWeight::contextual(0.5)
            .with_context("work".to_string(), 0.8)
            .with_context("leisure".to_string(), 0.3);

        // Test base weight (no context)
        assert_eq!(weight.value(None), 0.5);

        // Test context-specific weights
        assert_eq!(weight.value(Some("work")), 0.8);
        assert_eq!(weight.value(Some("leisure")), 0.3);

        // Test unknown context falls back to base
        assert_eq!(weight.value(Some("unknown")), 0.5);
    }

    /// Test attentional weight
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Attentional] --> B[Current Value]
    ///     B --> C[Clamp to Range]
    ///     C --> D[Update Attention]
    /// ```
    #[test]
    fn test_attentional_weight() {
        let mut weight = DimensionWeight::attentional(0.5, 0.1, 0.9);

        // Test initial value
        assert_eq!(weight.value(None), 0.5);

        // Test updating attention
        weight.update_attention(0.7);
        assert_eq!(weight.value(None), 0.7);

        // Test clamping to max
        weight.update_attention(1.5);
        assert_eq!(weight.value(None), 0.9);

        // Test clamping to min
        weight.update_attention(-0.5);
        assert_eq!(weight.value(None), 0.1);
    }

    /// Test attentional weight creation with clamping
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create with Out-of-Range] --> B[Auto-Clamp]
    ///     B --> C[Within Min-Max]
    /// ```
    #[test]
    fn test_attentional_creation_clamping() {
        // Current value above max
        let weight1 = DimensionWeight::attentional(1.5, 0.0, 1.0);
        assert_eq!(weight1.value(None), 1.0);

        // Current value below min
        let weight2 = DimensionWeight::attentional(-0.5, 0.0, 1.0);
        assert_eq!(weight2.value(None), 0.0);

        // Normal case
        let weight3 = DimensionWeight::attentional(0.5, 0.0, 1.0);
        assert_eq!(weight3.value(None), 0.5);
    }

    /// Test update_attention on non-attentional weights
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Non-Attentional Weight] --> B[Update Attention]
    ///     B --> C[No Effect]
    /// ```
    #[test]
    fn test_update_attention_on_non_attentional() {
        let mut constant = DimensionWeight::constant(0.5);
        constant.update_attention(0.8);
        assert_eq!(constant.value(None), 0.5); // Should not change

        let mut contextual = DimensionWeight::contextual(0.5);
        contextual.update_attention(0.8);
        assert_eq!(contextual.value(None), 0.5); // Should not change
    }

    /// Test complex contextual weight scenario
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Multiple Contexts] --> B[Override Base]
    ///     B --> C[Chain Building]
    ///     C --> D[Verify All Contexts]
    /// ```
    #[test]
    fn test_complex_contextual_weight() {
        let weight = DimensionWeight::contextual(0.5)
            .with_context("morning".to_string(), 0.3)
            .with_context("afternoon".to_string(), 0.6)
            .with_context("evening".to_string(), 0.8)
            .with_context("night".to_string(), 0.2);

        // Test all contexts
        assert_eq!(weight.value(Some("morning")), 0.3);
        assert_eq!(weight.value(Some("afternoon")), 0.6);
        assert_eq!(weight.value(Some("evening")), 0.8);
        assert_eq!(weight.value(Some("night")), 0.2);

        // Test that last value wins if same context added multiple times
        let weight2 = DimensionWeight::contextual(0.5)
            .with_context("test".to_string(), 0.3)
            .with_context("test".to_string(), 0.7); // Overrides previous

        assert_eq!(weight2.value(Some("test")), 0.7);
    }

    /// Test edge cases with extreme values
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Extreme Values] --> B[Negative Weights]
    ///     B --> C[Zero Weights]
    ///     C --> D[Large Weights]
    /// ```
    #[test]
    fn test_extreme_values() {
        // Negative weights (valid in some contexts)
        let negative = DimensionWeight::constant(-0.5);
        assert_eq!(negative.value(None), -0.5);

        // Zero weight
        let zero = DimensionWeight::constant(0.0);
        assert_eq!(zero.value(None), 0.0);

        // Large weight
        let large = DimensionWeight::constant(1000.0);
        assert_eq!(large.value(None), 1000.0);

        // Attentional with negative range
        let negative_attention = DimensionWeight::attentional(-0.5, -1.0, 0.0);
        assert_eq!(negative_attention.value(None), -0.5);
    }
}
