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
