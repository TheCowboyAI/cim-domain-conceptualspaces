//! Actual mathematical calculations for conceptual spaces
//!
//! This module implements the real mathematical algorithms for conceptual space
//! operations, replacing simplified placeholders with proper calculations.

pub mod voronoi;
pub mod convex_hull;
pub mod density_estimation;
pub mod dimension_reduction;
pub mod clustering;

pub use voronoi::{VoronoiCalculator, VoronoiDiagram};
pub use convex_hull::{ConvexHullCalculator, ConvexHull3D};
pub use density_estimation::{KernelDensityEstimator, DensityField};
pub use dimension_reduction::{PCAReducer, TSNEReducer, DimensionReducer};
pub use clustering::{DBSCANClustering, KMeansClustering, ClusteringAlgorithm}; 