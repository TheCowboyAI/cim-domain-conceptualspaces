//! Advanced similarity algorithms for conceptual spaces
//!
//! This module implements sophisticated similarity measures beyond basic distance metrics,
//! including semantic similarity, contextual similarity, and domain-specific measures.

use crate::{ConceptualPoint, ConceptualError, ConceptualResult, DistanceMetric, ConceptualSpace};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Advanced similarity computation engine
pub struct SimilarityEngine {
    /// Base distance metric
    pub base_metric: DistanceMetric,

    /// Context-dependent similarity weights
    pub context_weights: HashMap<String, Vec<f64>>,

    /// Semantic similarity cache
    similarity_cache: HashMap<(Uuid, Uuid), f64>,
}

impl SimilarityEngine {
    /// Create a new similarity engine
    pub fn new(base_metric: DistanceMetric) -> Self {
        Self {
            base_metric,
            context_weights: HashMap::new(),
            similarity_cache: HashMap::new(),
        }
    }

    /// Add context-specific weights
    pub fn add_context_weights(&mut self, context: String, weights: Vec<f64>) {
        self.context_weights.insert(context, weights);
    }

    /// Calculate basic similarity (inverse of distance)
    pub fn basic_similarity(&self, a: &ConceptualPoint, b: &ConceptualPoint) -> ConceptualResult<f64> {
        let distance = self.base_metric.calculate(a, b)?;
        Ok(1.0 / (1.0 + distance))
    }

    /// Calculate context-aware similarity
    pub fn contextual_similarity(
        &self, 
        a: &ConceptualPoint, 
        b: &ConceptualPoint, 
        context: Option<&str>
    ) -> ConceptualResult<f64> {
        match context.and_then(|c| self.context_weights.get(c)) {
            Some(weights) => {
                let weighted_distance = a.weighted_distance(b, weights, 2.0)?;
                Ok(1.0 / (1.0 + weighted_distance))
            },
            None => self.basic_similarity(a, b)
        }
    }

    /// Calculate semantic similarity using embedding-like approach
    pub fn semantic_similarity(&self, a: &ConceptualPoint, b: &ConceptualPoint) -> ConceptualResult<f64> {
        // Check cache first
        if let (Some(id_a), Some(id_b)) = (a.id, b.id) {
            let cache_key = if id_a < id_b { (id_a, id_b) } else { (id_b, id_a) };
            if let Some(&cached_similarity) = self.similarity_cache.get(&cache_key) {
                return Ok(cached_similarity);
            }
        }

        // Compute cosine similarity
        let dot_product = a.coordinates.dot(&b.coordinates);
        let norm_a = a.coordinates.norm();
        let norm_b = b.coordinates.norm();

        if norm_a == 0.0 || norm_b == 0.0 {
            return Ok(0.0);
        }

        let cosine_similarity = dot_product / (norm_a * norm_b);
        
        // Convert from [-1, 1] to [0, 1] range
        let similarity = (cosine_similarity + 1.0) / 2.0;

        Ok(similarity)
    }

    /// Calculate adaptive similarity that learns from feedback
    pub fn adaptive_similarity(
        &mut self,
        a: &ConceptualPoint,
        b: &ConceptualPoint,
        feedback: Option<f64>
    ) -> ConceptualResult<f64> {
        let current_similarity = self.semantic_similarity(a, b)?;

        // If feedback is provided, update the cache with weighted average
        if let (Some(id_a), Some(id_b), Some(feedback_score)) = (a.id, b.id, feedback) {
            let cache_key = if id_a < id_b { (id_a, id_b) } else { (id_b, id_a) };
            
            // Learning rate for adaptation
            let learning_rate = 0.1;
            let updated_similarity = current_similarity * (1.0 - learning_rate) + 
                                   feedback_score * learning_rate;
            
            self.similarity_cache.insert(cache_key, updated_similarity);
            Ok(updated_similarity)
        } else {
            Ok(current_similarity)
        }
    }

    /// Clear the similarity cache
    pub fn clear_cache(&mut self) {
        self.similarity_cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.similarity_cache.len(), self.similarity_cache.capacity())
    }
}

/// Advanced similarity algorithms implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedSimilarity;

impl AdvancedSimilarity {
    /// Calculate Gärdenfors-style natural category similarity
    ///
    /// This implements the conceptual spaces theory where similarity is based on
    /// distance within convex regions representing natural categories
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Point A] --> C[Find Region]
    ///     B[Point B] --> C
    ///     C --> D{Same Region?}
    ///     D -->|Yes| E[High Similarity]
    ///     D -->|No| F[Distance-based Similarity]
    /// ```
    pub fn category_based_similarity(
        point_a: &ConceptualPoint,
        point_b: &ConceptualPoint,
        space: &ConceptualSpace,
    ) -> ConceptualResult<f64> {
        let regions_a = space.find_containing_regions(point_a);
        let regions_b = space.find_containing_regions(point_b);

        // Check if points share any regions
        let shared_regions = regions_a.iter()
            .any(|region_a| regions_b.iter().any(|region_b| region_a.id == region_b.id));

        if shared_regions {
            // Points in same category are highly similar
            let distance = space.metric.distance(point_a, point_b)?;
            Ok(0.9 + 0.1 / (1.0 + distance)) // High base similarity + distance adjustment
        } else {
            // Standard distance-based similarity
            let distance = space.metric.distance(point_a, point_b)?;
            Ok(1.0 / (1.0 + distance))
        }
    }

    /// Calculate prototype-based similarity
    ///
    /// Similarity is determined by distance to category prototypes
    pub fn prototype_similarity(
        point: &ConceptualPoint,
        prototype: &ConceptualPoint,
        space: &ConceptualSpace,
    ) -> ConceptualResult<f64> {
        let distance_to_prototype = space.metric.distance(point, prototype)?;
        
        // Exponential decay from prototype
        Ok((-distance_to_prototype).exp())
    }

    /// Calculate salience-weighted similarity
    ///
    /// Some dimensions may be more salient (important) for similarity judgments
    pub fn salience_weighted_similarity(
        point_a: &ConceptualPoint,
        point_b: &ConceptualPoint,
        salience_weights: &[f64],
    ) -> ConceptualResult<f64> {
        if salience_weights.len() != point_a.coordinates.len() {
            return Err(ConceptualError::InvalidDimension(
                "Salience weights must match point dimensions".to_string()
            ));
        }

        let mut weighted_distance = 0.0;
        let mut total_weight = 0.0;

        for i in 0..point_a.coordinates.len() {
            let diff = point_a.coordinates[i] - point_b.coordinates[i];
            let weight = salience_weights[i];
            weighted_distance += weight * diff.powi(2);
            total_weight += weight;
        }

        if total_weight == 0.0 {
            return Ok(0.0);
        }

        let normalized_distance = (weighted_distance / total_weight).sqrt();
        Ok(1.0 / (1.0 + normalized_distance))
    }

    /// Calculate feature-based similarity
    ///
    /// Similarity based on shared features rather than geometric distance
    pub fn feature_similarity(
        features_a: &HashMap<String, f64>,
        features_b: &HashMap<String, f64>,
    ) -> ConceptualResult<f64> {
        let all_features: std::collections::HashSet<_> = features_a.keys()
            .chain(features_b.keys())
            .collect();

        if all_features.is_empty() {
            return Ok(0.0);
        }

        let feature_count = all_features.len();
        let mut similarity_sum = 0.0;
        
        for feature in &all_features {
            let value_a = features_a.get(*feature).unwrap_or(&0.0);
            let value_b = features_b.get(*feature).unwrap_or(&0.0);
            
            // Tversky-like feature similarity
            let min_val = value_a.min(*value_b);
            let max_val = value_a.max(*value_b);
            
            if max_val > 0.0 {
                similarity_sum += min_val / max_val;
            }
        }

        Ok(similarity_sum / feature_count as f64)
    }

    /// Calculate temporal similarity for dynamic concepts
    ///
    /// Concepts that change over time may have different similarity patterns
    pub fn temporal_similarity(
        trajectory_a: &[ConceptualPoint],
        trajectory_b: &[ConceptualPoint],
        metric: &DistanceMetric,
    ) -> ConceptualResult<f64> {
        if trajectory_a.is_empty() || trajectory_b.is_empty() {
            return Ok(0.0);
        }

        // Dynamic Time Warping-like approach
        let len_a = trajectory_a.len();
        let len_b = trajectory_b.len();
        let mut dtw_matrix = vec![vec![f64::INFINITY; len_b]; len_a];

        // Initialize first cell
        dtw_matrix[0][0] = metric.calculate(&trajectory_a[0], &trajectory_b[0])?;

        // Initialize first row and column
        for i in 1..len_a {
            dtw_matrix[i][0] = dtw_matrix[i-1][0] + metric.calculate(&trajectory_a[i], &trajectory_b[0])?;
        }
        for j in 1..len_b {
            dtw_matrix[0][j] = dtw_matrix[0][j-1] + metric.calculate(&trajectory_a[0], &trajectory_b[j])?;
        }

        // Fill the DTW matrix
        for i in 1..len_a {
            for j in 1..len_b {
                let cost = metric.calculate(&trajectory_a[i], &trajectory_b[j])?;
                dtw_matrix[i][j] = cost + dtw_matrix[i-1][j]
                    .min(dtw_matrix[i][j-1])
                    .min(dtw_matrix[i-1][j-1]);
            }
        }

        let dtw_distance = dtw_matrix[len_a-1][len_b-1];
        Ok(1.0 / (1.0 + dtw_distance))
    }

    /// Calculate multi-level similarity
    ///
    /// Combine different similarity measures at different levels of abstraction
    pub fn multi_level_similarity(
        point_a: &ConceptualPoint,
        point_b: &ConceptualPoint,
        space: &ConceptualSpace,
        levels: &[f64], // Weights for different abstraction levels
    ) -> ConceptualResult<f64> {
        if levels.is_empty() {
            return Ok(0.0);
        }

        let mut total_similarity = 0.0;
        let mut total_weight = 0.0;

        // Level 0: Basic geometric similarity
        if levels.len() > 0 {
            let geometric_sim = {
                let distance = space.metric.distance(point_a, point_b)?;
                1.0 / (1.0 + distance)
            };
            total_similarity += levels[0] * geometric_sim;
            total_weight += levels[0];
        }

        // Level 1: Category-based similarity
        if levels.len() > 1 {
            let category_sim = Self::category_based_similarity(point_a, point_b, space)?;
            total_similarity += levels[1] * category_sim;
            total_weight += levels[1];
        }

        // Level 2: Contextual similarity (simplified)
        if levels.len() > 2 {
            let contextual_sim = {
                // Use cosine similarity as a proxy for contextual similarity
                let dot_product = point_a.coordinates.dot(&point_b.coordinates);
                let norm_a = point_a.coordinates.norm();
                let norm_b = point_b.coordinates.norm();
                
                if norm_a > 0.0 && norm_b > 0.0 {
                    (dot_product / (norm_a * norm_b) + 1.0) / 2.0
                } else {
                    0.0
                }
            };
            total_similarity += levels[2] * contextual_sim;
            total_weight += levels[2];
        }

        if total_weight > 0.0 {
            Ok(total_similarity / total_weight)
        } else {
            Ok(0.0)
        }
    }
} 