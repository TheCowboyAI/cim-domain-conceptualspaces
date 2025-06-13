//! Conceptual space as a topological space with convex regions
//!
//! Based on Gärdenfors' theory of conceptual spaces, this module implements
//! a proper topological space where:
//! - The space has a metric topology induced by quality dimensions
//! - Regions representing natural concepts are convex
//! - The space forms a natural shape based on the distribution of points

use crate::{ConceptualError, ConceptualResult};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// A unique identifier for a conceptual space
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConceptualSpaceId(pub Uuid);

impl ConceptualSpaceId {
    /// Create a new random space ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ConceptualSpaceId {
    fn default() -> Self {
        Self::new()
    }
}

/// A unique identifier for a dimension
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DimensionId(pub Uuid);

impl DimensionId {
    /// Create a new random dimension ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DimensionId {
    fn default() -> Self {
        Self::new()
    }
}

// DimensionWeight moved to value_objects module
use crate::DimensionWeight;

/// A point in the topological conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualPoint {
    /// The n-dimensional coordinates
    pub coordinates: DVector<f64>,

    /// Maps dimension IDs to coordinate indices
    pub dimension_map: HashMap<DimensionId, usize>,

    /// Optional point identifier
    pub id: Option<Uuid>,
}

impl ConceptualPoint {
    /// Create a new conceptual point
    pub fn new(coordinates: Vec<f64>, dimension_map: HashMap<DimensionId, usize>) -> Self {
        Self {
            coordinates: DVector::from_vec(coordinates),
            dimension_map,
            id: Some(Uuid::new_v4()),
        }
    }

    /// Get the value for a specific dimension
    pub fn get_dimension_value(&self, dimension_id: &DimensionId) -> Option<f64> {
        self.dimension_map
            .get(dimension_id)
            .and_then(|&idx| self.coordinates.get(idx).copied())
    }

    /// Calculate weighted Minkowski distance to another point
    pub fn weighted_distance(
        &self,
        other: &ConceptualPoint,
        weights: &[f64],
        p: f64,
    ) -> ConceptualResult<f64> {
        if self.coordinates.len() != other.coordinates.len() {
            return Err(ConceptualError::InvalidPoint(
                "Points have different dimensions".to_string()
            ));
        }

        if weights.len() != self.coordinates.len() {
            return Err(ConceptualError::InvalidDimension(
                "Weight vector has incorrect length".to_string()
            ));
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

// ConvexRegion and Hyperplane moved to value_objects module
use crate::{ConvexRegion, Hyperplane};

/// Metric structure for the topological space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualMetric {
    /// Dimension weights for the metric
    pub dimension_weights: Vec<DimensionWeight>,

    /// Minkowski parameter (1 = Manhattan, 2 = Euclidean, ∞ = Chebyshev)
    pub minkowski_p: f64,

    /// Context for weight evaluation
    pub current_context: Option<String>,
}

impl ConceptualMetric {
    /// Create a new metric with uniform weights
    pub fn uniform(num_dimensions: usize, minkowski_p: f64) -> Self {
        Self {
            dimension_weights: vec![DimensionWeight::Constant(1.0); num_dimensions],
            minkowski_p,
            current_context: None,
        }
    }

    /// Get current weight values
    pub fn get_weights(&self) -> Vec<f64> {
        self.dimension_weights.iter()
            .map(|w| w.value(self.current_context.as_deref()))
            .collect()
    }

    /// Calculate distance between two points using this metric
    pub fn distance(&self, p1: &ConceptualPoint, p2: &ConceptualPoint) -> ConceptualResult<f64> {
        let weights = self.get_weights();
        p1.weighted_distance(p2, &weights, self.minkowski_p)
    }

    /// Create an open ball (neighborhood) around a point
    pub fn open_ball(&self, center: &ConceptualPoint, radius: f64) -> OpenBall {
        OpenBall {
            center: center.clone(),
            radius,
            metric: self.clone(),
        }
    }
}

/// An open ball in the topological space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenBall {
    /// Center of the ball
    pub center: ConceptualPoint,

    /// Radius of the ball
    pub radius: f64,

    /// Metric used for distance calculation
    pub metric: ConceptualMetric,
}

impl OpenBall {
    /// Check if a point is contained in this open ball
    pub fn contains(&self, point: &ConceptualPoint) -> ConceptualResult<bool> {
        let distance = self.metric.distance(&self.center, point)?;
        Ok(distance < self.radius)
    }
}

/// A topological conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualSpace {
    /// Unique identifier for this space
    pub id: ConceptualSpaceId,

    /// Name of the conceptual space
    pub name: String,

    /// The dimensions that define this space
    pub dimension_ids: Vec<DimensionId>,

    /// Metric structure for the space
    pub metric: ConceptualMetric,

    /// Convex regions within this space
    pub regions: HashMap<Uuid, ConvexRegion>,

    /// All points in the space (forms the point cloud)
    pub points: HashMap<Uuid, ConceptualPoint>,
}

impl ConceptualSpace {
    /// Create a new conceptual space
    pub fn new(name: String, dimension_ids: Vec<DimensionId>, metric: ConceptualMetric) -> Self {
        Self {
            id: ConceptualSpaceId::new(),
            name,
            dimension_ids,
            metric,
            regions: HashMap::new(),
            points: HashMap::new(),
        }
    }

    /// Add a point to the space
    pub fn add_point(&mut self, point: ConceptualPoint) -> ConceptualResult<Uuid> {
        let id = point.id.unwrap_or_else(Uuid::new_v4);
        self.points.insert(id, point);
        Ok(id)
    }

    /// Add a convex region to the space
    pub fn add_region(&mut self, region: ConvexRegion) -> ConceptualResult<()> {
        // Verify the region is actually convex
        let sample_points: Vec<_> = region.member_points.iter()
            .filter_map(|id| self.points.get(id))
            .cloned()
            .collect();

        if !sample_points.is_empty() && !region.is_convex(&sample_points) {
            return Err(ConceptualError::InvalidDimension(
                "Region is not convex".to_string()
            ));
        }

        self.regions.insert(region.id, region);
        Ok(())
    }

    /// Find all regions that contain a given point
    pub fn find_containing_regions(&self, point: &ConceptualPoint) -> Vec<&ConvexRegion> {
        self.regions.values()
            .filter(|region| region.contains(point))
            .collect()
    }

    /// Find k-nearest neighbors to a point
    pub fn k_nearest_neighbors(&self, point: &ConceptualPoint, k: usize) -> ConceptualResult<Vec<(&Uuid, f64)>> {
        let mut distances: Vec<_> = self.points.iter()
            .filter_map(|(id, p)| {
                self.metric.distance(point, p).ok().map(|d| (id, d))
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        distances.truncate(k);

        Ok(distances)
    }

    /// Compute the Voronoi cell for a prototype
    pub fn voronoi_cell(&self, prototype: &ConceptualPoint) -> ConceptualResult<Vec<&ConceptualPoint>> {
        let mut cell_points = Vec::new();

        for point in self.points.values() {
            let dist_to_prototype = self.metric.distance(point, prototype)?;

            // Check if this prototype is the nearest
            let mut is_nearest = true;
            for other_region in self.regions.values() {
                if other_region.prototype.id != prototype.id {
                    let dist_to_other = self.metric.distance(point, &other_region.prototype)?;
                    if dist_to_other < dist_to_prototype {
                        is_nearest = false;
                        break;
                    }
                }
            }

            if is_nearest {
                cell_points.push(point);
            }
        }

        Ok(cell_points)
    }

    /// Test if the space satisfies the axioms of a metric space
    pub fn verify_metric_axioms(&self, sample_size: usize) -> ConceptualResult<bool> {
        let points: Vec<_> = self.points.values().take(sample_size).collect();

        for i in 0..points.len() {
            for j in 0..points.len() {
                let d_ij = self.metric.distance(points[i], points[j])?;

                // Non-negativity
                if d_ij < 0.0 {
                    return Ok(false);
                }

                // Identity of indiscernibles
                if i == j && d_ij != 0.0 {
                    return Ok(false);
                }

                // Symmetry
                let d_ji = self.metric.distance(points[j], points[i])?;
                if (d_ij - d_ji).abs() > f64::EPSILON {
                    return Ok(false);
                }

                // Triangle inequality
                for k in 0..points.len() {
                    let d_ik = self.metric.distance(points[i], points[k])?;
                    let d_kj = self.metric.distance(points[k], points[j])?;
                    if d_ij > d_ik + d_kj + f64::EPSILON {
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true)
    }
}

