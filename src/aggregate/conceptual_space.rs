//! ConceptualSpace aggregate root for the Conceptual Spaces domain

use crate::{
    ConceptualSpace, ConceptualSpaceId, ConceptualPoint, ConvexRegion,
    DimensionId, ConceptualMetric, ConceptualError, ConceptualResult,
};
use cim_domain::{AggregateRoot, DomainError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The aggregate root for conceptual spaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptualSpaceAggregate {
    /// The underlying conceptual space
    space: ConceptualSpace,

    /// Version for optimistic concurrency control
    version: u64,

    /// Whether this aggregate has been deleted
    deleted: bool,
}

impl ConceptualSpaceAggregate {
    /// Create a new conceptual space aggregate
    pub fn new(name: String, dimension_ids: Vec<DimensionId>, metric: ConceptualMetric) -> Self {
        Self {
            space: ConceptualSpace::new(name, dimension_ids, metric),
            version: 0,
            deleted: false,
        }
    }

    /// Get the conceptual space ID
    pub fn space_id(&self) -> ConceptualSpaceId {
        self.space.id
    }

    /// Add a point to the conceptual space
    pub fn add_point(&mut self, point: ConceptualPoint) -> ConceptualResult<Uuid> {
        if self.deleted {
            return Err(ConceptualError::DomainError(DomainError::InvalidOperation {
                reason: "Cannot add point to deleted aggregate".to_string(),
            }));
        }

        let id = self.space.add_point(point)?;
        self.version += 1;
        Ok(id)
    }

    /// Add a convex region to the space
    pub fn add_region(&mut self, region: ConvexRegion) -> ConceptualResult<()> {
        if self.deleted {
            return Err(ConceptualError::DomainError(DomainError::InvalidOperation {
                reason: "Cannot add region to deleted aggregate".to_string(),
            }));
        }

        self.space.add_region(region)?;
        self.version += 1;
        Ok(())
    }

    /// Find all regions containing a point
    pub fn find_containing_regions(&self, point: &ConceptualPoint) -> Vec<&ConvexRegion> {
        if self.deleted {
            return Vec::new();
        }

        self.space.find_containing_regions(point)
    }

    /// Find k-nearest neighbors
    pub fn k_nearest_neighbors(&self, point: &ConceptualPoint, k: usize) -> ConceptualResult<Vec<(&Uuid, f64)>> {
        if self.deleted {
            return Err(ConceptualError::DomainError(DomainError::InvalidOperation {
                reason: "Cannot query deleted aggregate".to_string(),
            }));
        }

        self.space.k_nearest_neighbors(point, k)
    }

    /// Get current metric weights
    pub fn get_metric_weights(&self) -> Vec<f64> {
        if self.deleted {
            return Vec::new();
        }
        
        self.space.metric.dimension_weights
            .iter()
            .map(|w| w.value(None))
            .collect()
    }

    /// Update the metric weights
    pub fn update_metric_weights(&mut self, weights: Vec<f64>) -> ConceptualResult<()> {
        if self.deleted {
            return Err(ConceptualError::DomainError(DomainError::InvalidOperation {
                reason: "Cannot update weights on deleted aggregate".to_string(),
            }));
        }

        if weights.len() != self.space.metric.dimension_weights.len() {
            return Err(ConceptualError::InvalidDimension(
                "Weight vector has incorrect length".to_string()
            ));
        }

        // Convert f64 weights to DimensionWeight::Constant
        self.space.metric.dimension_weights = weights
            .into_iter()
            .map(crate::DimensionWeight::constant)
            .collect();
        
        self.version += 1;
        Ok(())
    }

    /// Mark the aggregate as deleted
    pub fn delete(&mut self) -> ConceptualResult<()> {
        if self.deleted {
            return Err(ConceptualError::DomainError(DomainError::InvalidOperation {
                reason: "Aggregate is already deleted".to_string(),
            }));
        }

        self.deleted = true;
        self.version += 1;
        Ok(())
    }

    /// Get a reference to the underlying conceptual space
    pub fn space(&self) -> &ConceptualSpace {
        &self.space
    }
}

impl AggregateRoot for ConceptualSpaceAggregate {
    type Id = ConceptualSpaceId;

    fn id(&self) -> Self::Id {
        self.space.id
    }

    fn version(&self) -> u64 {
        self.version
    }

    fn increment_version(&mut self) {
        self.version += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_metric() -> ConceptualMetric {
        ConceptualMetric {
            dimension_weights: vec![
                crate::DimensionWeight::constant(1.0),
                crate::DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0,
            current_context: None,
        }
    }

    fn create_test_point() -> ConceptualPoint {
        let mut dimension_map = HashMap::new();
        dimension_map.insert(DimensionId::new(), 0);
        dimension_map.insert(DimensionId::new(), 1);

        ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![1.0, 2.0]),
            dimension_map,
            id: Some(Uuid::new_v4()),
        }
    }

    /// Test aggregate creation
    /// 
    /// ```mermaid
    /// graph TD
    ///     A[Create Aggregate] --> B[Verify Initial State]
    ///     B --> C[Check Version = 0]
    ///     C --> D[Check Not Deleted]
    /// ```
    #[test]
    fn test_aggregate_creation() {
        let dimensions = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();
        let aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        assert_eq!(aggregate.version(), 0);
        assert!(!aggregate.deleted);
        assert_eq!(aggregate.space().name, "Test Space");
    }

    /// Test adding points to aggregate
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Add Point] --> B[Version Increments]
    ///     B --> C[Point Added to Space]
    ///     C --> D[Returns Valid ID]
    /// ```
    #[test]
    fn test_add_point() {
        let dimensions = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();
        let mut aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        let point = create_test_point();
        let result = aggregate.add_point(point.clone());

        assert!(result.is_ok());
        assert_eq!(aggregate.version(), 1);
    }

    /// Test operations on deleted aggregate
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Delete Aggregate] --> B[Try Add Point]
    ///     B --> C[Should Fail]
    ///     C --> D[Try Add Region]
    ///     D --> E[Should Fail]
    /// ```
    #[test]
    fn test_deleted_aggregate_operations() {
        let dimensions = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();
        let mut aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        // Delete the aggregate
        assert!(aggregate.delete().is_ok());
        assert_eq!(aggregate.version(), 1);

        // Try to add a point - should fail
        let point = create_test_point();
        let result = aggregate.add_point(point);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConceptualError::DomainError(DomainError::InvalidOperation { .. })
        ));

        // Try to add a region - should fail
        let region = ConvexRegion::from_prototype(create_test_point());
        let result = aggregate.add_region(region);
        assert!(result.is_err());

        // Try to delete again - should fail
        let result = aggregate.delete();
        assert!(result.is_err());
    }

    /// Test weight updates
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Update Weights] --> B[Version Increments]
    ///     B --> C[Weights Changed]
    ///     C --> D[Invalid Length Fails]
    /// ```
    #[test]
    fn test_update_weights() {
        let dimensions = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();
        let mut aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        // Update with valid weights
        let new_weights = vec![0.5, 0.8];
        assert!(aggregate.update_metric_weights(new_weights.clone()).is_ok());
        assert_eq!(aggregate.version(), 1);
        assert_eq!(aggregate.get_metric_weights(), new_weights);

        // Update with invalid length - should fail
        let invalid_weights = vec![0.5]; // Wrong length
        let result = aggregate.update_metric_weights(invalid_weights);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConceptualError::InvalidDimension(_)
        ));
    }

    /// Test k-nearest neighbors query
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Add Points] --> B[Query KNN]
    ///     B --> C[Returns Nearest]
    ///     C --> D[Correct Order]
    /// ```
    #[test]
    fn test_k_nearest_neighbors() {
        let dim1 = DimensionId::new();
        let dim2 = DimensionId::new();
        let dimensions = vec![dim1, dim2];
        let metric = create_test_metric();
        let mut aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        let mut dimension_map = HashMap::new();
        dimension_map.insert(dim1, 0);
        dimension_map.insert(dim2, 1);

        // Add some points
        let point1 = ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![0.0, 0.0]),
            dimension_map: dimension_map.clone(),
            id: Some(Uuid::new_v4()),
        };
        let point2 = ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![1.0, 1.0]),
            dimension_map: dimension_map.clone(),
            id: Some(Uuid::new_v4()),
        };
        let point3 = ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![2.0, 2.0]),
            dimension_map: dimension_map.clone(),
            id: Some(Uuid::new_v4()),
        };

        aggregate.add_point(point1.clone()).unwrap();
        aggregate.add_point(point2.clone()).unwrap();
        aggregate.add_point(point3.clone()).unwrap();

        // Query nearest to origin
        let query_point = ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![0.1, 0.1]),
            dimension_map,
            id: None,
        };

        let neighbors = aggregate.k_nearest_neighbors(&query_point, 2).unwrap();
        assert_eq!(neighbors.len(), 2);
        // First should be closest to origin
        assert!(neighbors[0].1 < neighbors[1].1);
    }

    /// Test region operations
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Region] --> B[Add to Space]
    ///     B --> C[Find Containing]
    ///     C --> D[Verify Results]
    /// ```
    #[test]
    fn test_region_operations() {
        let dimensions = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();
        let mut aggregate = ConceptualSpaceAggregate::new(
            "Test Space".to_string(),
            dimensions,
            metric,
        );

        // Create a region
        let prototype = create_test_point();
        let mut region = ConvexRegion::from_prototype(prototype.clone());
        region.name = Some("Test Region".to_string());

        // Add the region
        assert!(aggregate.add_region(region).is_ok());
        assert_eq!(aggregate.version(), 1);

        // Find containing regions
        let containing = aggregate.find_containing_regions(&prototype);
        assert_eq!(containing.len(), 1);
        assert_eq!(containing[0].name.as_ref().unwrap(), "Test Region");
    }
}
