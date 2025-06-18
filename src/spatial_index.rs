//! Spatial indexing for efficient conceptual space operations
//!
//! This module provides spatial data structures for fast nearest neighbor search,
//! range queries, and region-based operations in high-dimensional conceptual spaces.

use crate::{ConceptualPoint, ConceptualResult, DistanceMetric};
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use uuid::Uuid;

/// Trait for spatial index implementations
pub trait SpatialIndex {
    /// Insert a point into the index
    fn insert(&mut self, point: ConceptualPoint) -> ConceptualResult<()>;

    /// Remove a point from the index
    fn remove(&mut self, point_id: &Uuid) -> ConceptualResult<bool>;

    /// Find k nearest neighbors to a query point
    fn k_nearest_neighbors(&self, query: &ConceptualPoint, k: usize) -> ConceptualResult<Vec<(Uuid, f64)>>;

    /// Find all points within a radius
    fn range_search(&self, center: &ConceptualPoint, radius: f64) -> ConceptualResult<Vec<Uuid>>;

    /// Get the number of points in the index
    fn size(&self) -> usize;

    /// Clear all points from the index
    fn clear(&mut self);
}

/// A simple R-tree implementation for conceptual spaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RTreeIndex {
    /// Points stored in the index
    points: Vec<ConceptualPoint>,

    /// Distance metric for calculations
    metric: DistanceMetric,
}

impl RTreeIndex {
    /// Create a new R-tree index
    pub fn new(metric: DistanceMetric) -> Self {
        Self {
            points: Vec::new(),
            metric,
        }
    }
}

impl SpatialIndex for RTreeIndex {
    fn insert(&mut self, point: ConceptualPoint) -> ConceptualResult<()> {
        self.points.push(point);
        Ok(())
    }

    fn remove(&mut self, point_id: &Uuid) -> ConceptualResult<bool> {
        if let Some(pos) = self.points.iter().position(|p| p.id == Some(*point_id)) {
            self.points.remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn k_nearest_neighbors(&self, query: &ConceptualPoint, k: usize) -> ConceptualResult<Vec<(Uuid, f64)>> {
        let mut distances: Vec<_> = self.points.iter()
            .filter_map(|point| {
                if let Some(id) = point.id {
                    self.metric.calculate(query, point).ok().map(|d| (id, d))
                } else {
                    None
                }
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
        distances.truncate(k);
        Ok(distances)
    }

    fn range_search(&self, center: &ConceptualPoint, radius: f64) -> ConceptualResult<Vec<Uuid>> {
        let mut results = Vec::new();
        
        for point in &self.points {
            if let Some(id) = point.id {
                let distance = self.metric.calculate(center, point)?;
                if distance <= radius {
                    results.push(id);
                }
            }
        }

        Ok(results)
    }

    fn size(&self) -> usize {
        self.points.len()
    }

    fn clear(&mut self) {
        self.points.clear();
    }
}

/// A KD-tree implementation for conceptual spaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdTreeIndex {
    /// Root node of the KD-tree
    root: Option<Box<KdTreeNode>>,

    /// Number of dimensions
    dimensions: usize,

    /// Distance metric for calculations
    metric: DistanceMetric,

    /// Total number of points
    point_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KdTreeNode {
    /// The point stored at this node
    point: ConceptualPoint,

    /// Splitting dimension
    split_dim: usize,

    /// Left child (points with smaller values in split_dim)
    left: Option<Box<KdTreeNode>>,

    /// Right child (points with larger values in split_dim)
    right: Option<Box<KdTreeNode>>,
}

impl KdTreeIndex {
    /// Create a new KD-tree index
    pub fn new(dimensions: usize, metric: DistanceMetric) -> Self {
        Self {
            root: None,
            dimensions,
            metric,
            point_count: 0,
        }
    }

    /// Build the tree from a collection of points
    pub fn build_from_points(&mut self, mut points: Vec<ConceptualPoint>) -> ConceptualResult<()> {
        self.point_count = points.len();
        self.root = self.build_recursive(&mut points, 0);
        Ok(())
    }

    fn build_recursive(&self, points: &mut [ConceptualPoint], depth: usize) -> Option<Box<KdTreeNode>> {
        if points.is_empty() {
            return None;
        }

        let split_dim = depth % self.dimensions;

        // Sort points by the current dimension
        points.sort_by(|a, b| {
            let a_val = a.coordinates.get(split_dim).unwrap_or(&0.0);
            let b_val = b.coordinates.get(split_dim).unwrap_or(&0.0);
            a_val.partial_cmp(b_val).unwrap_or(Ordering::Equal)
        });

        let median = points.len() / 2;
        let median_point = points[median].clone();

        let left = if median > 0 {
            self.build_recursive(&mut points[..median], depth + 1)
        } else {
            None
        };

        let right = if median + 1 < points.len() {
            self.build_recursive(&mut points[median + 1..], depth + 1)
        } else {
            None
        };

        Some(Box::new(KdTreeNode {
            point: median_point,
            split_dim,
            left,
            right,
        }))
    }
}

impl SpatialIndex for KdTreeIndex {
    fn insert(&mut self, point: ConceptualPoint) -> ConceptualResult<()> {
        let old_root = self.root.take();
        self.root = Some(self.insert_recursive(old_root, point, 0));
        self.point_count += 1;
        Ok(())
    }

    fn remove(&mut self, _point_id: &Uuid) -> ConceptualResult<bool> {
        // KD-tree removal is complex and typically requires rebuilding
        Ok(false)
    }

    fn k_nearest_neighbors(&self, query: &ConceptualPoint, k: usize) -> ConceptualResult<Vec<(Uuid, f64)>> {
        let mut best = BinaryHeap::new();
        
        if let Some(ref root) = self.root {
            self.search_knn_recursive(root, query, k, &mut best, 0)?;
        }

        let mut result = Vec::new();
        while let Some(neighbor) = best.pop() {
            result.push((neighbor.id, neighbor.distance));
        }
        result.reverse(); // BinaryHeap is max-heap, we want min distances first
        Ok(result)
    }

    fn range_search(&self, center: &ConceptualPoint, radius: f64) -> ConceptualResult<Vec<Uuid>> {
        let mut results = Vec::new();
        
        if let Some(ref root) = self.root {
            self.search_range_recursive(root, center, radius, &mut results, 0)?;
        }

        Ok(results)
    }

    fn size(&self) -> usize {
        self.point_count
    }

    fn clear(&mut self) {
        self.root = None;
        self.point_count = 0;
    }
}

#[derive(Debug, Clone)]
struct KdNeighbor {
    id: Uuid,
    distance: f64,
}

impl PartialEq for KdNeighbor {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for KdNeighbor {}

impl PartialOrd for KdNeighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for KdNeighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for max-heap behavior
        other.distance.partial_cmp(&self.distance).unwrap_or(Ordering::Equal)
    }
}

impl KdTreeIndex {
    fn insert_recursive(&self, node: Option<Box<KdTreeNode>>, point: ConceptualPoint, depth: usize) -> Box<KdTreeNode> {
        match node {
            None => Box::new(KdTreeNode {
                point,
                split_dim: depth % self.dimensions,
                left: None,
                right: None,
            }),
            Some(mut existing) => {
                let split_dim = depth % self.dimensions;
                let point_val = point.coordinates.get(split_dim).unwrap_or(&0.0);
                let node_val = existing.point.coordinates.get(split_dim).unwrap_or(&0.0);

                if point_val < node_val {
                    existing.left = Some(self.insert_recursive(existing.left.take(), point, depth + 1));
                } else {
                    existing.right = Some(self.insert_recursive(existing.right.take(), point, depth + 1));
                }
                existing
            }
        }
    }

    fn search_knn_recursive(
        &self,
        node: &KdTreeNode,
        query: &ConceptualPoint,
        k: usize,
        best: &mut BinaryHeap<KdNeighbor>,
        depth: usize,
    ) -> ConceptualResult<()> {
        if let Some(id) = node.point.id {
            let distance = self.metric.calculate(query, &node.point)?;
            
            if best.len() < k {
                best.push(KdNeighbor { id, distance });
            } else if let Some(worst) = best.peek() {
                if distance < worst.distance {
                    best.pop();
                    best.push(KdNeighbor { id, distance });
                }
            }
        }

        let split_dim = depth % self.dimensions;
        let query_val = query.coordinates.get(split_dim).unwrap_or(&0.0);
        let node_val = node.point.coordinates.get(split_dim).unwrap_or(&0.0);

        let (near_child, far_child) = if query_val < node_val {
            (&node.left, &node.right)
        } else {
            (&node.right, &node.left)
        };

        // Search near child first
        if let Some(ref child) = near_child {
            self.search_knn_recursive(child, query, k, best, depth + 1)?;
        }

        // Check if we need to search far child
        let dimension_distance = (query_val - node_val).abs();
        if best.len() < k || (best.peek().map(|n| dimension_distance < n.distance).unwrap_or(true)) {
            if let Some(ref child) = far_child {
                self.search_knn_recursive(child, query, k, best, depth + 1)?;
            }
        }

        Ok(())
    }

    fn search_range_recursive(
        &self,
        node: &KdTreeNode,
        center: &ConceptualPoint,
        radius: f64,
        results: &mut Vec<Uuid>,
        depth: usize,
    ) -> ConceptualResult<()> {
        if let Some(id) = node.point.id {
            let distance = self.metric.calculate(center, &node.point)?;
            if distance <= radius {
                results.push(id);
            }
        }

        let split_dim = depth % self.dimensions;
        let center_val = center.coordinates.get(split_dim).unwrap_or(&0.0);
        let node_val = node.point.coordinates.get(split_dim).unwrap_or(&0.0);
        let dimension_distance = (center_val - node_val).abs();

        // Search both children if the splitting plane intersects the search radius
        if dimension_distance <= radius {
            if let Some(ref left) = node.left {
                self.search_range_recursive(left, center, radius, results, depth + 1)?;
            }
            if let Some(ref right) = node.right {
                self.search_range_recursive(right, center, radius, results, depth + 1)?;
            }
        } else {
            // Only search the side that the query point is on
            let child = if center_val < node_val {
                &node.left
            } else {
                &node.right
            };

            if let Some(ref child) = child {
                self.search_range_recursive(child, center, radius, results, depth + 1)?;
            }
        }

        Ok(())
    }
} 