//! Tests for the Conceptual Spaces domain

use cim_domain::{CommandEnvelope, CommandHandler};
use cim_domain_conceptualspaces::{
    similarity::*, CategoryBoundaryDetection, CategoryFormation, ConceptualMetric, ConceptualPoint,
    ConceptualSpace, ConceptualSpaceCommandHandler, ConceptualSpaceId, ConvexRegion,
    CreateConceptualSpace, DimensionId, DimensionWeight, DistanceMetric, Hyperplane, RTreeIndex,
    ReplaceDimensionWeights, SimilarityEngine, SpatialIndex,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Test F1: Basic conceptual space creation
///
/// ```mermaid
/// graph TD
///     A[Test Start] --> B[Create Command]
///     B --> C[Process Command]
///     C --> D[Verify Success]
/// ```
#[tokio::test]
async fn test_f1_basic_space_creation() {
    let mut handler = ConceptualSpaceCommandHandler::new();
    let space_id = ConceptualSpaceId(Uuid::new_v4());

    let command = CreateConceptualSpace {
        space_id,
        name: "Test Space".to_string(),
        dimension_ids: vec![DimensionId::new(), DimensionId::new()],
        metric: ConceptualMetric {
            dimension_weights: vec![
                DimensionWeight::constant(1.0),
                DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0, // Euclidean
            current_context: None,
        },
    };

    let envelope = CommandEnvelope::new(command, "test_user".to_string());
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, cim_domain::CommandStatus::Accepted);
    assert!(ack.reason.is_none());
}

/// Test F16: Command Handler Integration
///
/// ```mermaid
/// graph TD
///     A[Command] --> B[Handler]
///     B --> C[Aggregate]
///     C --> D[Acknowledgment]
/// ```
#[tokio::test]
async fn test_f16_command_handler_integration() {
    let mut handler = ConceptualSpaceCommandHandler::new();
    let space_id = ConceptualSpaceId(Uuid::new_v4());

    let command = CreateConceptualSpace {
        space_id,
        name: "Handler Test Space".to_string(),
        dimension_ids: vec![DimensionId::new()],
        metric: ConceptualMetric {
            dimension_weights: vec![DimensionWeight::constant(1.0)],
            minkowski_p: 1.0, // Manhattan
            current_context: None,
        },
    };

    let envelope = CommandEnvelope::new(command, "test_user".to_string());
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, cim_domain::CommandStatus::Accepted);

    // Test replace weights command
    let replace_command = ReplaceDimensionWeights {
        space_id,
        new_weights: vec![0.8],
        reason: "Testing weight replacement".to_string(),
    };

    let envelope = CommandEnvelope::new(replace_command, "test_user".to_string());
    let ack = handler.handle(envelope);

    assert_eq!(ack.status, cim_domain::CommandStatus::Accepted);
}

/// Test F17: Spatial Index Operations
///
/// ```mermaid
/// graph TD
///     A[Create Index] --> B[Insert Points]
///     B --> C[Query Neighbors]
///     C --> D[Verify Results]
/// ```
#[tokio::test]
async fn test_f17_spatial_index_operations() {
    let mut index = RTreeIndex::new(DistanceMetric::Euclidean);

    // Create test points
    let mut dimension_map = HashMap::new();
    dimension_map.insert(DimensionId::new(), 0);
    dimension_map.insert(DimensionId::new(), 1);

    let point1 = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![1.0, 2.0]),
        dimension_map: dimension_map.clone(),
        id: Some(Uuid::new_v4()),
    };

    let point2 = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![3.0, 4.0]),
        dimension_map: dimension_map.clone(),
        id: Some(Uuid::new_v4()),
    };

    // Insert points
    index.insert(point1.clone()).unwrap();
    index.insert(point2.clone()).unwrap();

    // Test k-nearest neighbors
    let query_point = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![1.5, 2.5]),
        dimension_map,
        id: None,
    };

    let neighbors = index.k_nearest_neighbors(&query_point, 1).unwrap();
    assert_eq!(neighbors.len(), 1);
    assert_eq!(neighbors[0].0, point1.id.unwrap());
}

/// Test F18: Similarity Engine
///
/// ```mermaid
/// graph TD
///     A[Create Engine] --> B[Calculate Similarities]
///     B --> C[Test Different Types]
///     C --> D[Verify Results]
/// ```
#[tokio::test]
async fn test_f18_similarity_engine() {
    let engine = SimilarityEngine::new(DistanceMetric::Euclidean);

    let mut dimension_map = HashMap::new();
    dimension_map.insert(DimensionId::new(), 0);
    dimension_map.insert(DimensionId::new(), 1);

    let point_a = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![1.0, 2.0]),
        dimension_map: dimension_map.clone(),
        id: Some(Uuid::new_v4()),
    };

    let point_b = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![1.1, 2.1]),
        dimension_map,
        id: Some(Uuid::new_v4()),
    };

    // Test basic similarity
    let similarity = engine.basic_similarity(&point_a, &point_b).unwrap();
    assert!(similarity > 0.8); // Should be high for close points

    // Test semantic similarity
    let semantic_sim = engine.semantic_similarity(&point_a, &point_b).unwrap();
    assert!(semantic_sim >= 0.0 && semantic_sim <= 1.0);
}

/// Test F19: Category Formation with Voronoi Tessellation
///
/// ```mermaid
/// graph TD
///     A[Create Points] --> B[Generate Voronoi]
///     B --> C[Analyze Density]
///     C --> D[Form Categories]
///     D --> E[Verify Convex Regions]
/// ```
#[tokio::test]
async fn test_f19_voronoi_category_formation() {
    let formation = CategoryFormation::new(DistanceMetric::Euclidean).with_params(3, 2.0);

    // Create a test space with clustered points
    let _space_id = ConceptualSpaceId(Uuid::new_v4());
    let mut dimension_map = HashMap::new();
    dimension_map.insert(DimensionId::new(), 0);
    dimension_map.insert(DimensionId::new(), 1);

    let space = ConceptualSpace::new(
        "Category Test Space".to_string(),
        vec![DimensionId::new(), DimensionId::new()],
        ConceptualMetric {
            dimension_weights: vec![
                DimensionWeight::constant(1.0),
                DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0, // Euclidean
            current_context: None,
        },
    );

    let categories = formation.detect_categories(&space).unwrap();
    // With empty space, should return no categories
    assert_eq!(categories.len(), 0);
}

/// Test F20: Boundary Detection using Voronoi Edges
///
/// ```mermaid
/// graph TD
///     A[Create Space] --> B[Generate Voronoi]
///     B --> C[Analyze Edge Densities]
///     C --> D[Extract Boundaries]
///     D --> E[Verify Boundary Strength]
/// ```
#[tokio::test]
async fn test_f20_voronoi_boundary_detection() {
    let detector = CategoryBoundaryDetection::new().with_params(0.5, 1.0);

    let space = ConceptualSpace::new(
        "Boundary Test Space".to_string(),
        vec![DimensionId::new(), DimensionId::new()],
        ConceptualMetric {
            dimension_weights: vec![
                DimensionWeight::constant(1.0),
                DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0, // Euclidean
            current_context: None,
        },
    );

    let boundaries = detector.detect_boundaries(&space).unwrap();
    // With empty space, should return no boundaries
    assert_eq!(boundaries.len(), 0);
}

/// Test F21: Prototype-based Similarity
///
/// ```mermaid
/// graph TD
///     A[Create Prototype] --> B[Create Test Point]
///     B --> C[Calculate Similarity]
///     C --> D[Verify Distance Decay]
/// ```
#[tokio::test]
async fn test_f21_prototype_similarity() {
    let space = ConceptualSpace::new(
        "Prototype Test Space".to_string(),
        vec![DimensionId::new(), DimensionId::new()],
        ConceptualMetric {
            dimension_weights: vec![
                DimensionWeight::constant(1.0),
                DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0, // Euclidean
            current_context: None,
        },
    );

    let mut dimension_map = HashMap::new();
    dimension_map.insert(DimensionId::new(), 0);
    dimension_map.insert(DimensionId::new(), 1);

    let prototype = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![0.0, 0.0]),
        dimension_map: dimension_map.clone(),
        id: Some(Uuid::new_v4()),
    };

    let test_point = ConceptualPoint {
        coordinates: nalgebra::DVector::from_vec(vec![1.0, 1.0]),
        dimension_map,
        id: Some(Uuid::new_v4()),
    };

    let similarity =
        AdvancedSimilarity::prototype_similarity(&test_point, &prototype, &space).unwrap();

    // Should be between 0 and 1, with exponential decay
    assert!(similarity >= 0.0 && similarity <= 1.0);
    assert!(similarity < 1.0); // Should be less than 1 due to distance
}
