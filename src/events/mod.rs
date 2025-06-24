//! Domain events for the Conceptual Spaces domain

mod space_created;
mod concept_added;
mod region_added;
mod weights_updated;

pub use space_created::*;
pub use concept_added::*;
pub use region_added::*;
pub use weights_updated::*;

use cim_domain::DomainEvent;
use crate::ConceptualSpaceId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Base trait for conceptual space events
pub trait ConceptualSpaceEvent: DomainEvent {
    /// Get the space ID this event relates to
    fn space_id(&self) -> ConceptualSpaceId;
}

/// All conceptual space domain events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptualSpaceDomainEvent {
    SpaceCreated(ConceptualSpaceCreated),
    ConceptAdded(ConceptAdded),
    RegionAdded(RegionAdded),
    WeightsRemoved(DimensionWeightsRemoved),
    WeightsAdded(DimensionWeightsAdded),
}

impl DomainEvent for ConceptualSpaceDomainEvent {
    fn event_type(&self) -> &'static str {
        match self {
            Self::SpaceCreated(_) => "ConceptualSpaceCreated",
            Self::ConceptAdded(_) => "ConceptAdded",
            Self::RegionAdded(_) => "RegionAdded",
            Self::WeightsRemoved(_) => "DimensionWeightsRemoved",
            Self::WeightsAdded(_) => "DimensionWeightsAdded",
        }
    }

    fn aggregate_id(&self) -> Uuid {
        match self {
            Self::SpaceCreated(e) => e.space_id.0,
            Self::ConceptAdded(e) => e.space_id.0,
            Self::RegionAdded(e) => e.space_id.0,
            Self::WeightsRemoved(e) => e.space_id.0,
            Self::WeightsAdded(e) => e.space_id.0,
        }
    }

    fn subject(&self) -> String {
        format!("conceptualspace.{}", self.event_type().to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ConceptualMetric, DimensionWeight, ConceptualPoint, ConvexRegion, DimensionId};
    use std::collections::HashMap;
    use uuid::Uuid;

    fn create_test_metric() -> ConceptualMetric {
        ConceptualMetric {
            dimension_weights: vec![
                DimensionWeight::constant(1.0),
                DimensionWeight::constant(1.0),
            ],
            minkowski_p: 2.0,
            current_context: None,
        }
    }

    /// Test space created event
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Event] --> B[Serialize]
    ///     B --> C[Deserialize]
    ///     C --> D[Verify Fields]
    /// ```
    #[test]
    fn test_space_created_event() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());
        let dimension_ids = vec![DimensionId::new(), DimensionId::new()];
        let metric = create_test_metric();

        let event = ConceptualSpaceCreated {
            space_id,
            name: "Test Space".to_string(),
            dimension_ids: dimension_ids.clone(),
            metric: metric.clone(),
        };

        // Test serialization/deserialization
        let json = serde_json::to_string(&event).unwrap();
        let deserialized: ConceptualSpaceCreated = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.space_id, space_id);
        assert_eq!(deserialized.name, "Test Space");
        assert_eq!(deserialized.dimension_ids.len(), 2);
    }

    /// Test concept added event
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Add Concept] --> B[Create Event]
    ///     B --> C[Verify Point]
    ///     C --> D[Check ID]
    /// ```
    #[test]
    fn test_concept_added_event() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());
        let concept_id = Uuid::new_v4();
        
        let mut dimension_map = HashMap::new();
        dimension_map.insert(DimensionId::new(), 0);
        dimension_map.insert(DimensionId::new(), 1);

        let point = ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![1.0, 2.0]),
            dimension_map,
            id: Some(concept_id),
        };

        let event = ConceptAdded {
            space_id,
            concept_id,
            point: point.clone(),
        };

        assert_eq!(event.space_id, space_id);
        assert_eq!(event.concept_id, concept_id);
        assert_eq!(event.point.coordinates[0], 1.0);
        assert_eq!(event.point.coordinates[1], 2.0);
    }

    /// Test region added event
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Region] --> B[Add to Space]
    ///     B --> C[Create Event]
    ///     C --> D[Verify Region]
    /// ```
    #[test]
    fn test_region_added_event() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());
        let region = ConvexRegion::from_prototype(ConceptualPoint {
            coordinates: nalgebra::DVector::from_vec(vec![0.0, 0.0]),
            dimension_map: HashMap::new(),
            id: None,
        });

        let event = RegionAdded {
            space_id,
            region: region.clone(),
        };

        assert_eq!(event.space_id, space_id);
        // ConvexRegion fields are tested in value object tests
    }

    /// Test dimension weights events
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Remove Weights] --> B[Add New Weights]
    ///     B --> C[Verify Removal]
    ///     C --> D[Verify Addition]
    /// ```
    #[test]
    fn test_dimension_weights_events() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());
        let old_weights = vec![1.0, 1.0];
        let new_weights = vec![0.5, 0.8];

        // Test removal event
        let remove_event = DimensionWeightsRemoved {
            space_id,
            removed_weights: old_weights.clone(),
            reason: "Testing weight removal".to_string(),
        };

        assert_eq!(remove_event.space_id, space_id);
        assert_eq!(remove_event.removed_weights, old_weights);
        assert_eq!(remove_event.reason, "Testing weight removal");

        // Test addition event
        let add_event = DimensionWeightsAdded {
            space_id,
            weights: new_weights.clone(),
            reason: "Testing weight update".to_string(),
        };

        assert_eq!(add_event.space_id, space_id);
        assert_eq!(add_event.weights, new_weights);
        assert_eq!(add_event.reason, "Testing weight update");
    }

    /// Test event enum wrapper
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Events] --> B[Wrap in Enum]
    ///     B --> C[Get Aggregate ID]
    ///     C --> D[Verify Correct ID]
    /// ```
    #[test]
    fn test_event_enum_aggregate_id() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());

        // Test each event type
        let events = vec![
            ConceptualSpaceDomainEvent::SpaceCreated(ConceptualSpaceCreated {
                space_id,
                name: "Test".to_string(),
                dimension_ids: vec![],
                metric: create_test_metric(),
            }),
            ConceptualSpaceDomainEvent::ConceptAdded(ConceptAdded {
                space_id,
                concept_id: Uuid::new_v4(),
                point: ConceptualPoint {
                    coordinates: nalgebra::DVector::from_vec(vec![]),
                    dimension_map: HashMap::new(),
                    id: None,
                },
            }),
            ConceptualSpaceDomainEvent::RegionAdded(RegionAdded {
                space_id,
                region: ConvexRegion::from_prototype(ConceptualPoint {
                    coordinates: nalgebra::DVector::from_vec(vec![]),
                    dimension_map: HashMap::new(),
                    id: None,
                }),
            }),
            ConceptualSpaceDomainEvent::WeightsRemoved(DimensionWeightsRemoved {
                space_id,
                removed_weights: vec![],
                reason: "Test removal".to_string(),
            }),
            ConceptualSpaceDomainEvent::WeightsAdded(DimensionWeightsAdded {
                space_id,
                weights: vec![],
                reason: "Test".to_string(),
            }),
        ];

        // All events should return the same aggregate ID
        for event in events {
            assert_eq!(event.aggregate_id(), space_id.0);
        }
    }

    /// Test event serialization round-trip
    ///
    /// ```mermaid
    /// graph TD
    ///     A[Create Event] --> B[Serialize to JSON]
    ///     B --> C[Deserialize]
    ///     C --> D[Compare Original]
    /// ```
    #[test]
    fn test_event_serialization_roundtrip() {
        let space_id = ConceptualSpaceId(Uuid::new_v4());
        let event = ConceptualSpaceDomainEvent::SpaceCreated(ConceptualSpaceCreated {
            space_id,
            name: "Serialization Test".to_string(),
            dimension_ids: vec![DimensionId::new(), DimensionId::new()],
            metric: create_test_metric(),
        });

        // Serialize to JSON
        let json = serde_json::to_string(&event).unwrap();

        // Deserialize back
        let deserialized: ConceptualSpaceDomainEvent = serde_json::from_str(&json).unwrap();

        // Verify it matches
        match (event, deserialized) {
            (
                ConceptualSpaceDomainEvent::SpaceCreated(orig),
                ConceptualSpaceDomainEvent::SpaceCreated(deser),
            ) => {
                assert_eq!(orig.space_id, deser.space_id);
                assert_eq!(orig.name, deser.name);
                assert_eq!(orig.dimension_ids.len(), deser.dimension_ids.len());
            }
            _ => panic!("Event type mismatch after deserialization"),
        }
    }
}
