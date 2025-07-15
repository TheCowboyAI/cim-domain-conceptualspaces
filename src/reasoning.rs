//! Conceptual reasoning capabilities for AI integration
//!
//! This module implements advanced reasoning algorithms that leverage conceptual spaces
//! for semantic understanding, analogy-making, and knowledge inference.

use crate::{
    ConceptualSpace, ConceptualPoint, ConceptualError, ConceptualResult,
    SimilarityEngine, CategoryFormation,
    DistanceMetric, RTreeIndex, SpatialIndex
};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use tracing::debug;

/// Conceptual reasoning engine for AI-powered semantic understanding
pub struct ConceptualReasoning {
    /// Similarity computation engine
    similarity_engine: SimilarityEngine,

    /// Category formation engine
    _category_formation: CategoryFormation,

    /// Spatial index for efficient reasoning
    spatial_index: RTreeIndex,

    /// Learning rate for adaptive reasoning
    learning_rate: f64,
}

impl ConceptualReasoning {
    /// Create a new reasoning engine
    pub fn new(metric: DistanceMetric) -> Self {
        Self {
            similarity_engine: SimilarityEngine::new(metric.clone()),
            _category_formation: CategoryFormation::new(metric.clone()),
            spatial_index: RTreeIndex::new(metric),
            learning_rate: 0.1,
        }
    }

    /// Configure learning parameters
    pub fn with_learning_rate(mut self, rate: f64) -> Self {
        self.learning_rate = rate;
        self
    }

    /// Perform analogical reasoning: A is to B as C is to ?
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Source A] --> V1[Vector A→B]
    ///     B[Source B] --> V1
    ///     C[Target C] --> V2[Apply Vector]
    ///     V1 --> V2
    ///     V2 --> D[Target D?]
    /// ```
    pub fn analogical_reasoning(
        &mut self,
        source_a: &ConceptualPoint,
        source_b: &ConceptualPoint,
        target_c: &ConceptualPoint,
        space: &ConceptualSpace,
    ) -> ConceptualResult<ConceptualPoint> {
        debug!("Performing analogical reasoning: A→B :: C→?");

        // Calculate the transformation vector from A to B
        let dimensions = source_a.coordinates.len();
        let mut transformation = DVector::zeros(dimensions);
        
        for i in 0..dimensions {
            transformation[i] = source_b.coordinates[i] - source_a.coordinates[i];
        }

        // Apply the transformation to C
        let mut target_d_coords = DVector::zeros(dimensions);
        for i in 0..dimensions {
            target_d_coords[i] = target_c.coordinates[i] + transformation[i];
        }

        // Create the analogical result
        let mut target_d = ConceptualPoint::new(
            target_d_coords.as_slice().to_vec(),
            target_c.dimension_map.clone()
        );

        // Find the nearest existing concept if needed
        if let Ok(nearest) = self.find_nearest_concept(&target_d, space, 1) {
            if !nearest.is_empty() {
                let (nearest_point, distance) = &nearest[0];
                
                // If very close to an existing concept, snap to it
                if distance < &0.1 {
                    target_d = nearest_point.clone();
                    debug!("Snapped to existing concept at distance {}", distance);
                }
            }
        }

        Ok(target_d)
    }

    /// Perform categorical inference
    ///
    /// Given a point, infer its likely category membership and properties
    pub fn categorical_inference(
        &mut self,
        point: &ConceptualPoint,
        space: &ConceptualSpace,
    ) -> ConceptualResult<CategoryInference> {
        debug!("Performing categorical inference for point");

        // Find containing regions
        let containing_regions = space.find_containing_regions(point);
        
        // Calculate membership probabilities
        let mut category_memberships = Vec::new();
        
        for region in &containing_regions {
            let distance_to_prototype = space.metric.distance(point, &region.prototype)?;
            let membership_strength = (-distance_to_prototype).exp(); // Exponential decay
            
            category_memberships.push(CategoryMembership {
                category_id: region.id,
                category_name: region.name.clone(),
                membership_strength,
                prototype_distance: distance_to_prototype,
            });
        }

        // Sort by membership strength
        category_memberships.sort_by(|a, b| 
            b.membership_strength.partial_cmp(&a.membership_strength).unwrap()
        );

        // Infer properties from strongest category
        let inferred_properties = if let Some(strongest) = category_memberships.first() {
            self.infer_properties_from_category(strongest.category_id, space)?
        } else {
            HashMap::new()
        };

        let confidence = self.calculate_inference_confidence(&category_memberships);
        
        Ok(CategoryInference {
            point: point.clone(),
            category_memberships,
            inferred_properties,
            confidence,
        })
    }

    /// Perform conceptual blending
    ///
    /// Blend multiple concepts to create new ones
    pub fn conceptual_blending(
        &mut self,
        concepts: &[ConceptualPoint],
        blend_weights: Option<&[f64]>,
        space: &ConceptualSpace,
    ) -> ConceptualResult<ConceptualBlend> {
        if concepts.is_empty() {
            return Err(ConceptualError::InvalidPoint(
                "Cannot blend empty concept list".to_string()
            ));
        }

        debug!("Blending {} concepts", concepts.len());

        let dimensions = concepts[0].coordinates.len();
        let default_weights = vec![1.0 / concepts.len() as f64; concepts.len()];
        let weights = blend_weights.unwrap_or(&default_weights);

        if weights.len() != concepts.len() {
            return Err(ConceptualError::InvalidDimension(
                "Blend weights must match number of concepts".to_string()
            ));
        }

        // Calculate weighted average
        let mut blended_coords = DVector::zeros(dimensions);
        
        for (concept, &weight) in concepts.iter().zip(weights.iter()) {
            for i in 0..dimensions {
                blended_coords[i] += concept.coordinates[i] * weight;
            }
        }

        // Create blended point
        let blended_point = ConceptualPoint::new(
            blended_coords.as_slice().to_vec(),
            concepts[0].dimension_map.clone()
        );

        // Analyze emergent properties
        let emergent_properties = self.detect_emergent_properties(&blended_point, concepts, space)?;

        // Calculate blend coherence
        let coherence = self.calculate_blend_coherence(&blended_point, concepts, space)?;

        Ok(ConceptualBlend {
            source_concepts: concepts.to_vec(),
            blend_weights: weights.to_vec(),
            blended_concept: blended_point,
            emergent_properties,
            coherence,
        })
    }

    /// Perform semantic path finding
    ///
    /// Find the most meaningful path between two concepts
    pub fn semantic_pathfinding(
        &mut self,
        start: &ConceptualPoint,
        goal: &ConceptualPoint,
        space: &ConceptualSpace,
        constraints: Option<PathConstraints>,
    ) -> ConceptualResult<SemanticPath> {
        debug!("Finding semantic path from start to goal");

        let constraints = constraints.unwrap_or_default();
        
        // A* search through conceptual space
        let mut open_set = vec![(0.0, start.clone(), vec![start.clone()])];
        let mut closed_set = HashSet::new();
        
        while let Some((_, current, path)) = open_set.pop() {
            if self.similarity_engine.basic_similarity(&current, goal)? > constraints.goal_threshold {
                let total_distance = self.calculate_path_distance(&path)?;
                let semantic_coherence = self.calculate_path_coherence(&path, space)?;
                return Ok(SemanticPath {
                    waypoints: path,
                    total_distance,
                    semantic_coherence,
                });
            }

            if path.len() >= constraints.max_path_length {
                continue;
            }

            // Add to closed set
            if let Some(id) = current.id {
                closed_set.insert(id);
            }

            // Find neighbors
            let neighbors = self.find_semantic_neighbors(&current, space, &constraints)?;
            
            for (neighbor, _) in neighbors {
                if let Some(id) = neighbor.id {
                    if closed_set.contains(&id) {
                        continue;
                    }
                }

                let mut new_path = path.clone();
                new_path.push(neighbor.clone());
                
                let g_score = self.calculate_path_distance(&new_path)?;
                let h_score = space.metric.distance(&neighbor, goal)?;
                let f_score = g_score + h_score;

                open_set.push((f_score, neighbor, new_path));
            }

            // Sort by f_score (ascending)
            open_set.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        }

        Err(ConceptualError::InvalidPoint(
            "No semantic path found within constraints".to_string()
        ))
    }

    /// Perform similarity-based retrieval
    pub fn similarity_retrieval(
        &mut self,
        query: &ConceptualPoint,
        space: &ConceptualSpace,
        k: usize,
        context: Option<&str>,
    ) -> ConceptualResult<Vec<SimilarityMatch>> {
        debug!("Retrieving {} similar concepts", k);

        let mut matches = Vec::new();
        
        for point in space.points.values() {
            let similarity = self.similarity_engine.contextual_similarity(
                query, point, context
            )?;

            matches.push(SimilarityMatch {
                concept: point.clone(),
                similarity_score: similarity,
                match_type: self.classify_match_type(similarity),
            });
        }

        // Sort by similarity (descending)
        matches.sort_by(|a, b| 
            b.similarity_score.partial_cmp(&a.similarity_score).unwrap()
        );

        matches.truncate(k);
        Ok(matches)
    }

    /// Learn from feedback to improve reasoning
    pub fn learn_from_feedback(
        &mut self,
        query: ReasoningQuery,
        _result: ReasoningResult,
        feedback: ReasoningFeedback,
    ) -> ConceptualResult<()> {
        debug!("Learning from reasoning feedback");

        // Update similarity engine based on feedback
        if let ReasoningQuery::Similarity(a, b) = &query {
            if let ReasoningFeedback::Similarity(target_score) = feedback {
                self.similarity_engine.adaptive_similarity(a, b, Some(target_score))?;
            }
        }

        // For now, just log the feedback
        if matches!(feedback, ReasoningFeedback::Positive) {
            debug!("Received positive feedback for reasoning");
        }

        Ok(())
    }

    // Helper methods

    fn find_nearest_concept(
        &mut self,
        point: &ConceptualPoint,
        space: &ConceptualSpace,
        k: usize,
    ) -> ConceptualResult<Vec<(ConceptualPoint, f64)>> {
        self.spatial_index.clear();
        
        for p in space.points.values() {
            self.spatial_index.insert(p.clone())?;
        }

        self.spatial_index.find_k_nearest(point, k)
    }

    fn infer_properties_from_category(
        &self,
        category_id: Uuid,
        space: &ConceptualSpace,
    ) -> ConceptualResult<HashMap<String, f64>> {
        let mut properties = HashMap::new();
        
        // Find the region
        if let Some(region) = space.regions.get(&category_id) {
            // Use prototype properties as inferred properties
            for &dim_idx in region.prototype.dimension_map.values() {
                // For now, just use the dimension index as a property name
                let value = region.prototype.coordinates[dim_idx];
                properties.insert(format!("dimension_{dim_idx}"), value);
            }
        }

        Ok(properties)
    }

    fn calculate_inference_confidence(&self, memberships: &[CategoryMembership]) -> f64 {
        if memberships.is_empty() {
            return 0.0;
        }

        // Confidence based on membership strength and clarity
        let max_strength = memberships.first().map(|m| m.membership_strength).unwrap_or(0.0);
        let second_strength = memberships.get(1).map(|m| m.membership_strength).unwrap_or(0.0);
        
        // High confidence if clear winner
        let clarity = max_strength - second_strength;
        (max_strength * clarity).min(1.0)
    }

    fn detect_emergent_properties(
        &self,
        blend: &ConceptualPoint,
        sources: &[ConceptualPoint],
        space: &ConceptualSpace,
    ) -> ConceptualResult<Vec<EmergentProperty>> {
        let mut properties = Vec::new();

        // Check if blend creates new category membership
        let blend_regions = space.find_containing_regions(blend);
        
        for region in &blend_regions {
            let mut is_novel = true;
            
            for source in sources {
                let source_regions = space.find_containing_regions(source);
                if source_regions.iter().any(|r| r.id == region.id) {
                    is_novel = false;
                    break;
                }
            }

            if is_novel {
                properties.push(EmergentProperty {
                    property_type: EmergentType::NovelCategory,
                    description: format!("Novel category membership: {:?}", region.name),
                    strength: 1.0,
                });
            }
        }

        // Check for cross-domain properties
        if sources.len() >= 2 {
            // Calculate if the blend bridges different regions
            let source_region_sets: Vec<_> = sources.iter()
                .map(|s| space.find_containing_regions(s))
                .collect();
            
            let mut all_different = true;
            for i in 0..source_region_sets.len()-1 {
                for j in i+1..source_region_sets.len() {
                    if source_region_sets[i].iter().any(|r1| 
                        source_region_sets[j].iter().any(|r2| r1.id == r2.id)
                    ) {
                        all_different = false;
                        break;
                    }
                }
            }
            
            if all_different {
                properties.push(EmergentProperty {
                    property_type: EmergentType::CrossDomain,
                    description: "Blend bridges different conceptual domains".to_string(),
                    strength: 0.8,
                });
            }
        }

        Ok(properties)
    }

    fn calculate_blend_coherence(
        &self,
        blend: &ConceptualPoint,
        sources: &[ConceptualPoint],
        space: &ConceptualSpace,
    ) -> ConceptualResult<f64> {
        let mut total_similarity = 0.0;
        let mut region_coherence = 0.0;
        
        // Basic similarity to sources
        for source in sources {
            total_similarity += self.similarity_engine.basic_similarity(blend, source)?;
        }

        // Check if blend maintains region coherence
        let blend_regions = space.find_containing_regions(blend);
        if !blend_regions.is_empty() {
            // Check how many sources share regions with the blend
            let mut shared_region_count = 0;
            for source in sources {
                let source_regions = space.find_containing_regions(source);
                if source_regions.iter().any(|sr| 
                    blend_regions.iter().any(|br| sr.id == br.id)
                ) {
                    shared_region_count += 1;
                }
            }
            region_coherence = shared_region_count as f64 / sources.len() as f64;
        }

        // Combine similarity and region coherence
        let similarity_score = total_similarity / sources.len() as f64;
        Ok(similarity_score * 0.7 + region_coherence * 0.3)
    }

    fn calculate_path_distance(&self, path: &[ConceptualPoint]) -> ConceptualResult<f64> {
        let mut total = 0.0;
        
        for window in path.windows(2) {
            let distance = window[0].coordinates.iter()
                .zip(window[1].coordinates.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            total += distance;
        }

        Ok(total)
    }

    fn calculate_path_coherence(
        &self,
        path: &[ConceptualPoint],
        space: &ConceptualSpace,
    ) -> ConceptualResult<f64> {
        if path.len() < 2 {
            return Ok(1.0);
        }

        let mut coherence_sum = 0.0;
        let mut region_transitions = 0;
        
        for window in path.windows(2) {
            let similarity = self.similarity_engine.basic_similarity(&window[0], &window[1])?;
            coherence_sum += similarity;
            
            // Check if we're transitioning between regions
            let regions1 = space.find_containing_regions(&window[0]);
            let regions2 = space.find_containing_regions(&window[1]);
            
            // Count transitions (fewer is better for coherence)
            if regions1.is_empty() != regions2.is_empty() {
                region_transitions += 1;
            } else if !regions1.is_empty() {
                let same_region = regions1.iter().any(|r1| 
                    regions2.iter().any(|r2| r1.id == r2.id)
                );
                if !same_region {
                    region_transitions += 1;
                }
            }
        }

        let similarity_coherence = coherence_sum / (path.len() - 1) as f64;
        let transition_penalty = region_transitions as f64 / (path.len() - 1) as f64;
        
        // Higher coherence with fewer region transitions
        Ok(similarity_coherence * (1.0 - transition_penalty * 0.3))
    }

    fn find_semantic_neighbors(
        &self,
        point: &ConceptualPoint,
        space: &ConceptualSpace,
        constraints: &PathConstraints,
    ) -> ConceptualResult<Vec<(ConceptualPoint, f64)>> {
        let mut neighbors = Vec::new();

        for candidate in space.points.values() {
            let distance = space.metric.distance(point, candidate)?;
            
            if distance <= constraints.max_step_size && distance > 0.0 {
                neighbors.push((candidate.clone(), distance));
            }
        }

        neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        neighbors.truncate(constraints.beam_width);
        
        Ok(neighbors)
    }

    fn classify_match_type(&self, similarity: f64) -> MatchType {
        if similarity > 0.9 {
            MatchType::Exact
        } else if similarity > 0.7 {
            MatchType::Close
        } else if similarity > 0.5 {
            MatchType::Related
        } else {
            MatchType::Distant
        }
    }
}

// Data structures for reasoning

#[derive(Debug, Clone)]
pub enum ReasoningQuery {
    Similarity(ConceptualPoint, ConceptualPoint),
    Analogy(ConceptualPoint, ConceptualPoint, ConceptualPoint),
    Categorization(ConceptualPoint),
}

#[derive(Debug, Clone)]
pub enum ReasoningResult {
    Similarity(f64),
    Analogy(ConceptualPoint),
    Categorization(Vec<CategoryMembership>),
}

#[derive(Debug, Clone)]
pub enum ReasoningFeedback {
    Positive,
    Negative,
    Similarity(f64),
    Correction(ReasoningResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInference {
    pub point: ConceptualPoint,
    pub category_memberships: Vec<CategoryMembership>,
    pub inferred_properties: HashMap<String, f64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryMembership {
    pub category_id: Uuid,
    pub category_name: Option<String>,
    pub membership_strength: f64,
    pub prototype_distance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualBlend {
    pub source_concepts: Vec<ConceptualPoint>,
    pub blend_weights: Vec<f64>,
    pub blended_concept: ConceptualPoint,
    pub emergent_properties: Vec<EmergentProperty>,
    pub coherence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentProperty {
    pub property_type: EmergentType,
    pub description: String,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergentType {
    NovelCategory,
    CrossDomain,
    Synergy,
    Contradiction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticPath {
    pub waypoints: Vec<ConceptualPoint>,
    pub total_distance: f64,
    pub semantic_coherence: f64,
}

#[derive(Debug, Clone)]
pub struct PathConstraints {
    pub max_path_length: usize,
    pub max_step_size: f64,
    pub goal_threshold: f64,
    pub beam_width: usize,
}

impl Default for PathConstraints {
    fn default() -> Self {
        Self {
            max_path_length: 10,
            max_step_size: 2.0,
            goal_threshold: 0.9,
            beam_width: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityMatch {
    pub concept: ConceptualPoint,
    pub similarity_score: f64,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchType {
    Exact,
    Close,
    Related,
    Distant,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DimensionId, ConceptualSpaceId, DistanceMetric};
    use std::collections::{HashMap, HashSet};
    use uuid::Uuid;

    fn create_test_space() -> ConceptualSpace {
        use crate::{ConceptualMetric, ConvexRegion, Hyperplane};
        use nalgebra::DVector;
        
        let dim1 = DimensionId::new();
        let dim2 = DimensionId::new();
        
        let mut space = ConceptualSpace::new(
            "Test Space".to_string(),
            vec![dim1, dim2],
            ConceptualMetric::uniform(2, 2.0), // Euclidean metric
        );

        // Add some test points
        let mut dim_map = HashMap::new();
        dim_map.insert(dim1, 0);
        dim_map.insert(dim2, 1);

        // Add test points
        let points = vec![
            (vec![0.2, 0.3], "point1"),
            (vec![0.4, 0.5], "point2"),
            (vec![0.6, 0.7], "point3"),
            (vec![0.8, 0.9], "point4"),
        ];

        for (coords, _name) in points {
            let point = ConceptualPoint::new(coords, dim_map.clone());
            space.add_point(point).unwrap();
        }

        // Add a test region
        let prototype = ConceptualPoint::new(vec![0.5, 0.5], dim_map.clone());
        let boundaries = vec![
            Hyperplane::new(DVector::from_vec(vec![1.0, 0.0]), 0.0),
            Hyperplane::new(DVector::from_vec(vec![-1.0, 0.0]), -1.0),
            Hyperplane::new(DVector::from_vec(vec![0.0, 1.0]), 0.0),
            Hyperplane::new(DVector::from_vec(vec![0.0, -1.0]), -1.0),
        ];

        let region = ConvexRegion {
            id: Uuid::new_v4(),
            prototype,
            boundaries,
            member_points: HashSet::new(),
            name: Some("Test Region".to_string()),
            description: Some("A test convex region".to_string()),
        };

        space.add_region(region).unwrap();

        space
    }

    #[test]
    fn test_analogical_reasoning() {
        let space = create_test_space();
        let mut reasoning = ConceptualReasoning::new(DistanceMetric::Euclidean);

        // Create test points
        let mut dim_map = HashMap::new();
        dim_map.insert(DimensionId::new(), 0);
        dim_map.insert(DimensionId::new(), 1);

        let a = ConceptualPoint::new(vec![0.2, 0.3], dim_map.clone());
        let b = ConceptualPoint::new(vec![0.4, 0.5], dim_map.clone());
        let c = ConceptualPoint::new(vec![0.6, 0.7], dim_map.clone());

        // A is to B as C is to D
        let d = reasoning.analogical_reasoning(&a, &b, &c, &space).unwrap();

        // D should be at (0.8, 0.9) following the pattern
        assert!((d.coordinates[0] - 0.8).abs() < 0.001);
        assert!((d.coordinates[1] - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_conceptual_blending() {
        let space = create_test_space();
        let mut reasoning = ConceptualReasoning::new(DistanceMetric::Euclidean);

        let mut dim_map = HashMap::new();
        dim_map.insert(DimensionId::new(), 0);
        dim_map.insert(DimensionId::new(), 1);

        let concept1 = ConceptualPoint::new(vec![0.2, 0.8], dim_map.clone());
        let concept2 = ConceptualPoint::new(vec![0.8, 0.2], dim_map.clone());

        let blend = reasoning.conceptual_blending(
            &[concept1, concept2],
            None,
            &space
        ).unwrap();

        // Blend should be at midpoint
        assert!((blend.blended_concept.coordinates[0] - 0.5).abs() < 0.001);
        assert!((blend.blended_concept.coordinates[1] - 0.5).abs() < 0.001);
        assert!(blend.coherence > 0.0);
    }
} 