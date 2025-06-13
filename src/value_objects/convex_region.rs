//! Convex region value object representing natural categories

use crate::{ConceptualPoint, ConceptualError, ConceptualResult};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

/// A hyperplane in the conceptual space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hyperplane {
    /// Normal vector to the hyperplane
    pub normal: DVector<f64>,

    /// Offset from origin
    pub offset: f64,
}

impl Hyperplane {
    /// Create a hyperplane from normal vector and offset
    pub fn new(normal: DVector<f64>, offset: f64) -> Self {
        Self { normal, offset }
    }

    /// Calculate signed distance from a point to the hyperplane
    /// Positive = point is on the side of the normal vector
    pub fn signed_distance(&self, point: &ConceptualPoint) -> f64 {
        self.normal.dot(&point.coordinates) - self.offset
    }

    /// Check if a point is on the positive side of the hyperplane
    pub fn contains_positive(&self, point: &ConceptualPoint) -> bool {
        self.signed_distance(point) >= 0.0
    }
}

/// A convex region in conceptual space representing a natural category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvexRegion {
    /// Unique identifier for this region
    pub id: Uuid,

    /// The prototype (most typical point) of this region
    pub prototype: ConceptualPoint,

    /// Hyperplanes defining the convex hull
    pub boundaries: Vec<Hyperplane>,

    /// Member points (for prototype updates)
    pub member_points: HashSet<Uuid>,

    /// Optional name for the region
    pub name: Option<String>,

    /// Optional description
    pub description: Option<String>,
}

impl ConvexRegion {
    /// Create a new convex region from a prototype
    pub fn from_prototype(prototype: ConceptualPoint) -> Self {
        Self {
            id: Uuid::new_v4(),
            prototype,
            boundaries: Vec::new(),
            member_points: HashSet::new(),
            name: None,
            description: None,
        }
    }

    /// Create a named convex region
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Add a description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Check if a point is within this convex region
    /// A point is inside if it's on the positive side of all boundary hyperplanes
    pub fn contains(&self, point: &ConceptualPoint) -> bool {
        self.boundaries.iter().all(|plane| plane.contains_positive(point))
    }

    /// Update the prototype based on member points
    pub fn update_prototype(&mut self, points: &[ConceptualPoint]) -> ConceptualResult<()> {
        if points.is_empty() {
            return Err(ConceptualError::InvalidPoint(
                "Cannot update prototype with no concepts".to_string()
            ));
        }

        // Calculate centroid as new prototype
        let dim = points[0].coordinates.len();
        let mut centroid = DVector::zeros(dim);

        for point in points {
            centroid += &point.coordinates;
        }

        centroid /= points.len() as f64;

        self.prototype.coordinates = centroid;
        Ok(())
    }

    /// Test convexity: for any two points in the region, all points on the line between them are also in the region
    pub fn is_convex(&self, sample_points: &[ConceptualPoint]) -> bool {
        for i in 0..sample_points.len() {
            for j in i+1..sample_points.len() {
                // Test several points along the line
                for t in [0.25, 0.5, 0.75] {
                    let interpolated = self.interpolate_points(&sample_points[i], &sample_points[j], t);
                    if !self.contains(&interpolated) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Interpolate between two points
    fn interpolate_points(&self, p1: &ConceptualPoint, p2: &ConceptualPoint, t: f64) -> ConceptualPoint {
        let coords = &p1.coordinates * (1.0 - t) + &p2.coordinates * t;
        ConceptualPoint {
            coordinates: coords,
            dimension_map: p1.dimension_map.clone(),
            id: None,
        }
    }

    /// Add a member point ID
    pub fn add_member(&mut self, concept_id: Uuid) {
        self.member_points.insert(concept_id);
    }

    /// Remove a member point ID
    pub fn remove_member(&mut self, concept_id: &Uuid) -> bool {
        self.member_points.remove(concept_id)
    }

    /// Get the number of members
    pub fn member_count(&self) -> usize {
        self.member_points.len()
    }
}
