//! Category formation and boundary detection algorithms using Voronoi tessellations
//!
//! This module implements algorithms for automatically detecting natural categories
//! and conceptual boundaries in conceptual spaces, following Gärdenfors' theory
//! of natural category formation through convex regions using Voronoi tessellations.

use crate::{
    ConceptualPoint, ConceptualSpace, ConvexRegion, Hyperplane, 
    ConceptualError, ConceptualResult, DistanceMetric, SpatialIndex, RTreeIndex
};
use nalgebra::DVector;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Voronoi-based category formation algorithms
pub struct CategoryFormation {
    /// Distance metric for calculations
    metric: DistanceMetric,

    /// Minimum points required to form a category
    min_points_per_category: usize,

    /// Maximum distance for points to be considered in same category
    max_category_radius: f64,

    /// Spatial index for efficient neighbor search
    spatial_index: RTreeIndex,
}

impl CategoryFormation {
    /// Create a new category formation engine
    pub fn new(metric: DistanceMetric) -> Self {
        Self {
            spatial_index: RTreeIndex::new(metric.clone()),
            metric,
            min_points_per_category: 3,
            max_category_radius: 2.0,
        }
    }

    /// Configure parameters
    pub fn with_params(
        mut self, 
        min_points: usize, 
        max_radius: f64
    ) -> Self {
        self.min_points_per_category = min_points;
        self.max_category_radius = max_radius;
        self
    }

    /// Add points to the spatial index for category formation
    pub fn add_points(&mut self, points: Vec<ConceptualPoint>) -> ConceptualResult<()> {
        for point in points {
            self.spatial_index.insert(point)?;
        }
        Ok(())
    }

    /// Detect natural categories using Voronoi tessellation and density-based clustering
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Points] --> B[Generate Voronoi Tessellation]
    ///     B --> C[Analyze Cell Density]
    ///     C --> D[Merge Adjacent Dense Cells]
    ///     D --> E[Form Convex Regions]
    ///     E --> F[Natural Categories]
    /// ```
    pub fn detect_categories(&self, space: &ConceptualSpace) -> ConceptualResult<Vec<ConvexRegion>> {
        let points: Vec<_> = space.points.values().cloned().collect();
        
        if points.len() < self.min_points_per_category {
            return Ok(Vec::new());
        }

        // Generate Voronoi tessellation
        let voronoi = self.generate_voronoi_tessellation(&points)?;
        
        // Analyze density and merge cells to form categories
        let categories = self.form_categories_from_voronoi(&voronoi, &points)?;

        Ok(categories)
    }

    /// Generate Voronoi tessellation from points
    fn generate_voronoi_tessellation(&self, points: &[ConceptualPoint]) -> ConceptualResult<VoronoiTessellation> {
        let mut cells = Vec::new();
        
        for (i, point) in points.iter().enumerate() {
            let cell = self.compute_voronoi_cell(point, points, i)?;
            cells.push(cell);
        }

        Ok(VoronoiTessellation {
            cells,
            seed_points: points.to_vec(),
        })
    }

    /// Compute a single Voronoi cell for a seed point
    fn compute_voronoi_cell(
        &self,
        seed: &ConceptualPoint,
        all_points: &[ConceptualPoint],
        seed_index: usize,
    ) -> ConceptualResult<VoronoiCell> {
        let mut boundaries = Vec::new();
        
        // For each other point, create a bisecting hyperplane
        for (i, other_point) in all_points.iter().enumerate() {
            if i == seed_index {
                continue;
            }

            let hyperplane = self.compute_bisecting_hyperplane(seed, other_point)?;
            boundaries.push(hyperplane);
        }

        // Calculate cell volume (approximation)
        let volume = self.estimate_cell_volume(seed, &boundaries)?;

        Ok(VoronoiCell {
            seed_point: seed.clone(),
            seed_index,
            boundaries,
            volume,
            density: 0.0, // Will be calculated later
        })
    }

    /// Compute bisecting hyperplane between two points
    fn compute_bisecting_hyperplane(
        &self,
        point1: &ConceptualPoint,
        point2: &ConceptualPoint,
    ) -> ConceptualResult<Hyperplane> {
        let dimensions = point1.coordinates.len();
        
        // Midpoint between the two points
        let mut midpoint = DVector::zeros(dimensions);
        for i in 0..dimensions {
            midpoint[i] = (point1.coordinates[i] + point2.coordinates[i]) / 2.0;
        }

        // Normal vector points from point1 to point2
        let mut normal = DVector::zeros(dimensions);
        for i in 0..dimensions {
            normal[i] = point2.coordinates[i] - point1.coordinates[i];
        }

        // Normalize the normal vector
        let magnitude = normal.norm();
        if magnitude > 0.0 {
            normal /= magnitude;
        }

        // Calculate the offset for the hyperplane equation
        let offset = normal.dot(&midpoint);

        Ok(Hyperplane::new(normal, offset))
    }

    /// Estimate volume of a Voronoi cell (simplified)
    fn estimate_cell_volume(
        &self,
        _seed: &ConceptualPoint,
        _boundaries: &[Hyperplane],
    ) -> ConceptualResult<f64> {
        // Simplified volume estimation
        // In a full implementation, this would compute the actual polytope volume
        Ok(1.0)
    }

    /// Form categories from Voronoi tessellation by merging dense regions
    fn form_categories_from_voronoi(
        &self,
        voronoi: &VoronoiTessellation,
        points: &[ConceptualPoint],
    ) -> ConceptualResult<Vec<ConvexRegion>> {
        // Calculate density for each cell
        let mut cells_with_density = voronoi.cells.clone();
        for cell in &mut cells_with_density {
            cell.density = self.calculate_cell_density(&cell.seed_point, points)?;
        }

        // Find high-density cells
        let avg_density = cells_with_density.iter()
            .map(|c| c.density)
            .sum::<f64>() / cells_with_density.len() as f64;
        
        let dense_cells: Vec<_> = cells_with_density.into_iter()
            .filter(|cell| cell.density > avg_density * 1.5) // 1.5x above average
            .collect();

        // Group adjacent dense cells into categories
        let cell_groups = self.group_adjacent_cells(&dense_cells)?;

        // Convert each group to a convex region
        let mut categories = Vec::new();
        for group in cell_groups {
            if group.len() >= self.min_points_per_category {
                let region = self.create_region_from_cell_group(&group)?;
                categories.push(region);
            }
        }

        Ok(categories)
    }

    /// Calculate density around a point using kernel density estimation
    fn calculate_cell_density(
        &self,
        center: &ConceptualPoint,
        all_points: &[ConceptualPoint],
    ) -> ConceptualResult<f64> {
        let bandwidth = self.max_category_radius / 2.0;
        let mut density = 0.0;

        for point in all_points {
            let distance = self.metric.calculate(center, point)?;
            
            // Gaussian kernel
            let kernel_value = (-0.5 * (distance / bandwidth).powi(2)).exp();
            density += kernel_value;
        }

        // Normalize by number of points and bandwidth
        density /= all_points.len() as f64 * bandwidth;
        
        Ok(density)
    }

    /// Group adjacent Voronoi cells based on proximity
    fn group_adjacent_cells(&self, cells: &[VoronoiCell]) -> ConceptualResult<Vec<Vec<VoronoiCell>>> {
        let mut groups = Vec::new();
        let mut visited = HashSet::new();

        for (i, _cell) in cells.iter().enumerate() {
            if visited.contains(&i) {
                continue;
            }

            let mut group = Vec::new();
            let mut to_visit = vec![i];

            while let Some(idx) = to_visit.pop() {
                if visited.contains(&idx) {
                    continue;
                }

                visited.insert(idx);
                group.push(cells[idx].clone());

                // Find adjacent cells
                for (j, other_cell) in cells.iter().enumerate() {
                    if !visited.contains(&j) {
                        let distance = self.metric.calculate(&cells[idx].seed_point, &other_cell.seed_point)?;
                        if distance <= self.max_category_radius {
                            to_visit.push(j);
                        }
                    }
                }
            }

            if !group.is_empty() {
                groups.push(group);
            }
        }

        Ok(groups)
    }

    /// Create a convex region from a group of Voronoi cells
    fn create_region_from_cell_group(&self, cells: &[VoronoiCell]) -> ConceptualResult<ConvexRegion> {
        if cells.is_empty() {
            return Err(ConceptualError::InvalidDimension(
                "Cannot create region from empty cell group".to_string()
            ));
        }

        // Calculate centroid of all seed points as prototype
        let dimensions = cells[0].seed_point.coordinates.len();
        let mut centroid = DVector::zeros(dimensions);
        
        for cell in cells {
            for (i, &coord) in cell.seed_point.coordinates.iter().enumerate() {
                centroid[i] += coord;
            }
        }
        centroid /= cells.len() as f64;

        // Create prototype point
        let mut dimension_map = HashMap::new();
        for (i, _) in centroid.iter().enumerate() {
            dimension_map.insert(crate::DimensionId::new(), i);
        }
        
        let prototype = ConceptualPoint::new(centroid.as_slice().to_vec(), dimension_map);

        // Compute convex hull of all cell boundaries
        let boundaries = self.compute_group_convex_hull(cells)?;

        // Collect member points
        let member_points: HashSet<_> = cells.iter()
            .filter_map(|cell| cell.seed_point.id)
            .collect();

        Ok(ConvexRegion {
            id: Uuid::new_v4(),
            prototype,
            boundaries,
            member_points,
            name: Some(format!("Voronoi Category {}", Uuid::new_v4())),
            description: Some("Category formed from Voronoi tessellation".to_string()),
        })
    }

    /// Compute convex hull boundaries for a group of cells
    fn compute_group_convex_hull(&self, cells: &[VoronoiCell]) -> ConceptualResult<Vec<Hyperplane>> {
        // Collect all seed points
        let points: Vec<_> = cells.iter().map(|c| &c.seed_point).collect();
        
        // For simplicity, create a bounding box that encompasses all points
        // In a full implementation, this would compute the actual convex hull
        self.compute_bounding_hyperplanes(&points)
    }

    /// Compute hyperplanes that form a bounding box around points
    fn compute_bounding_hyperplanes(&self, points: &[&ConceptualPoint]) -> ConceptualResult<Vec<Hyperplane>> {
        if points.is_empty() {
            return Ok(Vec::new());
        }

        let dimensions = points[0].coordinates.len();
        let mut boundaries = Vec::new();

        for dim in 0..dimensions {
            // Find min and max in this dimension
            let mut min_val = f64::INFINITY;
            let mut max_val = f64::NEG_INFINITY;

            for point in points {
                if let Some(val) = point.coordinates.get(dim) {
                    min_val = min_val.min(*val);
                    max_val = max_val.max(*val);
                }
            }

            // Add margin
            let margin = (max_val - min_val) * 0.1;
            min_val -= margin;
            max_val += margin;

            // Create hyperplanes for min and max bounds
            let mut normal_min = DVector::zeros(dimensions);
            normal_min[dim] = 1.0;
            boundaries.push(Hyperplane::new(normal_min, -min_val));

            let mut normal_max = DVector::zeros(dimensions);
            normal_max[dim] = -1.0;
            boundaries.push(Hyperplane::new(normal_max, max_val));
        }

        Ok(boundaries)
    }
}

/// Voronoi tessellation representation
#[derive(Debug, Clone)]
pub struct VoronoiTessellation {
    /// Individual Voronoi cells
    pub cells: Vec<VoronoiCell>,
    /// Original seed points
    pub seed_points: Vec<ConceptualPoint>,
}

/// A single cell in the Voronoi tessellation
#[derive(Debug, Clone)]
pub struct VoronoiCell {
    /// The seed point that generates this cell
    pub seed_point: ConceptualPoint,
    /// Index of the seed point in the original array
    pub seed_index: usize,
    /// Boundaries of the cell (bisecting hyperplanes)
    pub boundaries: Vec<Hyperplane>,
    /// Estimated volume of the cell
    pub volume: f64,
    /// Density of points in this cell's neighborhood
    pub density: f64,
}

/// Boundary detection algorithms for conceptual spaces using Voronoi tessellations
pub struct CategoryBoundaryDetection {
    /// Gradient threshold for boundary detection
    gradient_threshold: f64,

    /// Smoothing parameter for density estimation
    smoothing_factor: f64,
}

impl CategoryBoundaryDetection {
    /// Create a new boundary detection engine
    pub fn new() -> Self {
        Self {
            gradient_threshold: 0.5,
            smoothing_factor: 1.0,
        }
    }

    /// Configure parameters
    pub fn with_params(mut self, gradient_threshold: f64, smoothing_factor: f64) -> Self {
        self.gradient_threshold = gradient_threshold;
        self.smoothing_factor = smoothing_factor;
        self
    }

    /// Detect conceptual boundaries using Voronoi tessellation edges
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Conceptual Space] --> B[Generate Voronoi Tessellation]
    ///     B --> C[Analyze Edge Densities]
    ///     C --> D[Find High-Gradient Edges]
    ///     D --> E[Extract Boundary Lines]
    ///     E --> F[Category Boundaries]
    /// ```
    pub fn detect_boundaries(
        &self,
        space: &ConceptualSpace,
    ) -> ConceptualResult<Vec<ConceptualBoundary>> {
        let points: Vec<_> = space.points.values().collect();
        
        if points.is_empty() {
            return Ok(Vec::new());
        }

        // Generate Voronoi tessellation
        let voronoi = self.generate_voronoi_for_boundaries(&points)?;

        // Analyze edge densities and find boundaries
        let boundaries = self.extract_boundaries_from_voronoi(&voronoi)?;

        Ok(boundaries)
    }

    /// Generate Voronoi tessellation for boundary detection
    fn generate_voronoi_for_boundaries(
        &self,
        points: &[&ConceptualPoint],
    ) -> ConceptualResult<VoronoiTessellation> {
        let points_owned: Vec<_> = points.iter().cloned().cloned().collect();
        let mut cells = Vec::new();
        
        for (i, point) in points_owned.iter().enumerate() {
            let cell = self.compute_boundary_voronoi_cell(point, &points_owned, i)?;
            cells.push(cell);
        }

        Ok(VoronoiTessellation {
            cells,
            seed_points: points_owned,
        })
    }

    /// Compute Voronoi cell for boundary detection
    fn compute_boundary_voronoi_cell(
        &self,
        seed: &ConceptualPoint,
        all_points: &[ConceptualPoint],
        seed_index: usize,
    ) -> ConceptualResult<VoronoiCell> {
        let mut boundaries = Vec::new();
        let mut neighbors = Vec::new();
        
        // Find neighboring points and create bisecting planes
        for (i, other_point) in all_points.iter().enumerate() {
            if i == seed_index {
                continue;
            }

            let distance = seed.coordinates.iter()
                .zip(other_point.coordinates.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();

            // Only consider nearby points for boundary formation
            if distance < self.smoothing_factor * 3.0 {
                neighbors.push((i, distance));
                
                let hyperplane = self.compute_bisecting_hyperplane(seed, other_point)?;
                boundaries.push(hyperplane);
            }
        }

        // Calculate local density
        let density = neighbors.len() as f64 / (self.smoothing_factor.powi(seed.coordinates.len() as i32));

        Ok(VoronoiCell {
            seed_point: seed.clone(),
            seed_index,
            boundaries,
            volume: 1.0 / (density + 1e-6), // Inverse density approximates volume
            density,
        })
    }

    /// Compute bisecting hyperplane between two points
    fn compute_bisecting_hyperplane(
        &self,
        point1: &ConceptualPoint,
        point2: &ConceptualPoint,
    ) -> ConceptualResult<Hyperplane> {
        let dimensions = point1.coordinates.len();
        
        // Midpoint
        let mut midpoint = DVector::zeros(dimensions);
        for i in 0..dimensions {
            midpoint[i] = (point1.coordinates[i] + point2.coordinates[i]) / 2.0;
        }

        // Normal vector
        let mut normal = DVector::zeros(dimensions);
        for i in 0..dimensions {
            normal[i] = point2.coordinates[i] - point1.coordinates[i];
        }

        // Normalize
        let magnitude = normal.norm();
        if magnitude > 0.0 {
            normal /= magnitude;
        }

        let offset = normal.dot(&midpoint);

        Ok(Hyperplane::new(normal, offset))
    }

    /// Extract boundaries from Voronoi tessellation by analyzing density gradients
    fn extract_boundaries_from_voronoi(
        &self,
        voronoi: &VoronoiTessellation,
    ) -> ConceptualResult<Vec<ConceptualBoundary>> {
        let mut boundaries = Vec::new();

        // Find edges between cells with significant density differences
        for (i, cell1) in voronoi.cells.iter().enumerate() {
            for (j, cell2) in voronoi.cells.iter().enumerate() {
                if i >= j {
                    continue;
                }

                // Check if cells are adjacent (share a boundary)
                if self.cells_are_adjacent(cell1, cell2)? {
                    let density_diff = (cell1.density - cell2.density).abs();
                    let max_density = cell1.density.max(cell2.density);
                    
                    if max_density > 0.0 {
                        let gradient_strength = density_diff / max_density;
                        
                        if gradient_strength > self.gradient_threshold {
                            // Create boundary at the edge between cells
                            let boundary = self.create_boundary_from_edge(cell1, cell2, gradient_strength)?;
                            boundaries.push(boundary);
                        }
                    }
                }
            }
        }

        Ok(boundaries)
    }

    /// Check if two Voronoi cells are adjacent
    fn cells_are_adjacent(&self, cell1: &VoronoiCell, cell2: &VoronoiCell) -> ConceptualResult<bool> {
        // Two cells are adjacent if the distance between their seed points is small
        let distance = cell1.seed_point.coordinates.iter()
            .zip(cell2.seed_point.coordinates.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        Ok(distance <= self.smoothing_factor * 2.0)
    }

    /// Create a boundary from an edge between two cells
    fn create_boundary_from_edge(
        &self,
        cell1: &VoronoiCell,
        cell2: &VoronoiCell,
        strength: f64,
    ) -> ConceptualResult<ConceptualBoundary> {
        // Boundary position is midpoint between seed points
        let dimensions = cell1.seed_point.coordinates.len();
        let mut boundary_coords = Vec::with_capacity(dimensions);
        
        for i in 0..dimensions {
            let midpoint = (cell1.seed_point.coordinates[i] + cell2.seed_point.coordinates[i]) / 2.0;
            boundary_coords.push(midpoint);
        }

        let mut dimension_map = HashMap::new();
        for (i, _) in boundary_coords.iter().enumerate() {
            dimension_map.insert(crate::DimensionId::new(), i);
        }

        let position = ConceptualPoint::new(boundary_coords, dimension_map);

        Ok(ConceptualBoundary {
            id: Uuid::new_v4(),
            position,
            strength,
            boundary_type: BoundaryType::VoronoiEdge,
        })
    }
}

impl Default for CategoryBoundaryDetection {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a conceptual boundary in the space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualBoundary {
    /// Unique identifier
    pub id: Uuid,

    /// Position of the boundary
    pub position: ConceptualPoint,

    /// Strength of the boundary (0.0 - 1.0)
    pub strength: f64,

    /// Type of boundary
    pub boundary_type: BoundaryType,
}

/// Types of conceptual boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryType {
    /// Boundary detected from density gradients
    DensityGradient,

    /// Boundary from Voronoi tessellation edges
    VoronoiEdge,

    /// Boundary between convex regions
    RegionBoundary,

    /// Boundary detected from similarity patterns
    SimilarityBoundary,

    /// Manually specified boundary
    Manual,
}

// Extension to ConvexRegion to support the new boundary formation
impl ConvexRegion {
    /// Create a convex region from prototype with computed boundaries
    pub fn from_prototype_with_boundaries(
        prototype: ConceptualPoint,
        boundaries: Vec<Hyperplane>,
        member_points: HashSet<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            prototype,
            boundaries,
            member_points,
            name: Some("Generated Boundary".to_string()),
            description: Some("Boundary generated from Voronoi edge".to_string()),
        }
    }
} 