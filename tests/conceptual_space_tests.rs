//! Tests for conceptual space implementation

use cim_domain_conceptualspaces::{
    ConceptualSpace, ConceptualPoint, ConvexRegion, DimensionId,
    ConceptualMetric, DimensionWeight, QualityDimension, DimensionType,
    Hyperplane
};
use nalgebra::DVector;
use std::collections::HashMap;

/// User Story F11: As a developer, I want to create a topological conceptual space
/// so that I can represent domain concepts geometrically
///
/// ```mermaid
/// graph TD
///     A[Create Space] --> B[Add Dimensions]
///     B --> C[Define Metric]
///     C --> D[Verify Topology]
/// ```
#[test]
fn test_create_conceptual_space() {
    // Given: A set of quality dimensions
    let dim1 = DimensionId::new();
    let dim2 = DimensionId::new();
    let dim3 = DimensionId::new();
    let dimensions = vec![dim1, dim2, dim3];

    // And: A metric with uniform weights
    let metric = ConceptualMetric::uniform(3, 2.0); // Euclidean metric

    // When: Creating a conceptual space
    let space = ConceptualSpace::new("TestSpace".to_string(), dimensions, metric);

    // Then: The space should be properly initialized
    assert_eq!(space.name, "TestSpace");
    assert_eq!(space.dimension_ids.len(), 3);
    assert!(space.regions.is_empty());
    assert!(space.points.is_empty());
}

/// User Story F12: As a developer, I want to ensure regions are convex
/// so that natural categories form proper conceptual regions
///
/// ```mermaid
/// graph TD
///     A[Create Region] --> B[Add Points]
///     B --> C[Test Convexity]
///     C --> D{Is Convex?}
///     D -->|Yes| E[Valid Region]
///     D -->|No| F[Invalid Region]
/// ```
#[test]
fn test_convex_region_validation() {
    // Given: A prototype point
    let mut dimension_map = HashMap::new();
    dimension_map.insert(DimensionId::new(), 0);
    dimension_map.insert(DimensionId::new(), 1);

    let prototype = ConceptualPoint::new(vec![0.5, 0.5], dimension_map.clone());

    // When: Creating a convex region
    let mut region = ConvexRegion::from_prototype(prototype);

    // And: Adding boundary hyperplanes to form a square
    // (x >= 0, x <= 1, y >= 0, y <= 1)
    region.boundaries.push(Hyperplane::new(
        DVector::from_vec(vec![1.0, 0.0]), // Normal pointing right
        0.0 // x >= 0
    ));
    region.boundaries.push(Hyperplane::new(
        DVector::from_vec(vec![-1.0, 0.0]), // Normal pointing left
        -1.0 // x <= 1
    ));
    region.boundaries.push(Hyperplane::new(
        DVector::from_vec(vec![0.0, 1.0]), // Normal pointing up
        0.0 // y >= 0
    ));
    region.boundaries.push(Hyperplane::new(
        DVector::from_vec(vec![0.0, -1.0]), // Normal pointing down
        -1.0 // y <= 1
    ));

    // Then: Points inside the square should be contained
    let inside_point = ConceptualPoint::new(vec![0.3, 0.7], dimension_map.clone());
    assert!(region.contains(&inside_point));

    // And: Points outside should not be contained
    let outside_point = ConceptualPoint::new(vec![1.5, 0.5], dimension_map);
    assert!(!region.contains(&outside_point));
}

/// User Story F13: As a developer, I want to calculate distances in conceptual space
/// so that I can measure semantic similarity
///
/// ```mermaid
/// graph TD
///     A[Point A] --> C[Calculate Distance]
///     B[Point B] --> C
///     C --> D[Weighted Metric]
///     D --> E[Similarity Score]
/// ```
#[test]
fn test_weighted_distance_calculation() {
    // Given: Two points in 3D conceptual space
    let mut dimension_map = HashMap::new();
    let dim1 = DimensionId::new();
    let dim2 = DimensionId::new();
    let dim3 = DimensionId::new();
    dimension_map.insert(dim1, 0);
    dimension_map.insert(dim2, 1);
    dimension_map.insert(dim3, 2);

    let point1 = ConceptualPoint::new(vec![1.0, 2.0, 3.0], dimension_map.clone());
    let point2 = ConceptualPoint::new(vec![4.0, 6.0, 8.0], dimension_map);

    // And: Different weight configurations
    let uniform_weights = vec![1.0, 1.0, 1.0];
    let biased_weights = vec![2.0, 1.0, 0.5];

    // When: Calculating Euclidean distance (p=2)
    let uniform_distance = point1.weighted_distance(&point2, &uniform_weights, 2.0).unwrap();
    let biased_distance = point1.weighted_distance(&point2, &biased_weights, 2.0).unwrap();

    // Then: Distances should be calculated correctly
    // Uniform: sqrt((3)^2 + (4)^2 + (5)^2) = sqrt(50) ≈ 7.07
    assert!((uniform_distance - 7.071).abs() < 0.01);

    // Biased: sqrt(2*(3)^2 + 1*(4)^2 + 0.5*(5)^2) = sqrt(18 + 16 + 12.5) = sqrt(46.5) ≈ 6.82
    assert!((biased_distance - 6.819).abs() < 0.01);
}

/// User Story F14: As a developer, I want to verify metric space axioms
/// so that the conceptual space has valid mathematical properties
///
/// ```mermaid
/// graph TD
///     A[Metric Space] --> B[Non-negativity]
///     A --> C[Identity]
///     A --> D[Symmetry]
///     A --> E[Triangle Inequality]
/// ```
#[test]
fn test_metric_space_axioms() {
    // Given: A conceptual space with points
    let dimensions = vec![DimensionId::new(), DimensionId::new()];
    let metric = ConceptualMetric::uniform(2, 2.0);
    let mut space = ConceptualSpace::new("MetricTest".to_string(), dimensions, metric);

    // Add some test points
    let mut dim_map = HashMap::new();
    dim_map.insert(space.dimension_ids[0], 0);
    dim_map.insert(space.dimension_ids[1], 1);

    let points = vec![
        ConceptualPoint::new(vec![0.0, 0.0], dim_map.clone()),
        ConceptualPoint::new(vec![1.0, 0.0], dim_map.clone()),
        ConceptualPoint::new(vec![0.0, 1.0], dim_map.clone()),
        ConceptualPoint::new(vec![1.0, 1.0], dim_map.clone()),
    ];

    for point in points {
        space.add_point(point).unwrap();
    }

    // When: Verifying metric axioms
    let is_valid_metric = space.verify_metric_axioms(4).unwrap();

    // Then: The space should satisfy all metric axioms
    assert!(is_valid_metric);
}

/// User Story F15: As a developer, I want context-dependent dimension weights
/// so that similarity can adapt to different contexts
///
/// ```mermaid
/// graph TD
///     A[Base Weights] --> B{Context?}
///     B -->|Context A| C[Modified Weights A]
///     B -->|Context B| D[Modified Weights B]
///     B -->|None| E[Base Weights]
/// ```
#[test]
fn test_contextual_dimension_weights() {
    // Given: Contextual dimension weights
    let mut context_modifiers = HashMap::new();
    context_modifiers.insert("visual".to_string(), 2.0);
    context_modifiers.insert("semantic".to_string(), 0.5);

    let weight = DimensionWeight::Contextual {
        base_weight: 1.0,
        context_modifiers,
    };

    // When: Evaluating in different contexts
    let base_value = weight.value(None);
    let visual_value = weight.value(Some("visual"));
    let semantic_value = weight.value(Some("semantic"));
    let unknown_value = weight.value(Some("unknown"));

    // Then: Weights should adapt to context
    assert_eq!(base_value, 1.0);
    assert_eq!(visual_value, 2.0);
    assert_eq!(semantic_value, 0.5);
    assert_eq!(unknown_value, 1.0); // Falls back to base
}
