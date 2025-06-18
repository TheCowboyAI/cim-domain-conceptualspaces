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
