//! ConceptMap types for storing concepts in the Object Store

use crate::space::{ConceptualPoint, DimensionId};
use cid::Cid;
use petgraph::graph::{Graph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique identifier for a concept map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConceptMapId(Uuid);

impl ConceptMapId {
    /// Create a new random concept map ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from an existing UUID
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for ConceptMapId {
    fn default() -> Self {
        Self::new()
    }
}

/// Unique identifier for a context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContextId(Uuid);

impl Default for ContextId {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextId {
    /// Create a new random context ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from a string name (deterministic)
    pub fn from_name(name: &str) -> Self {
        Self(Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()))
    }
}

/// Well-known context IDs
impl ContextId {
    /// Identity context ID
    pub fn identity() -> Self {
        Self::from_name("identity.cim")
    }

    /// Security context ID
    pub fn security() -> Self {
        Self::from_name("security.cim")
    }

    /// Workflow context ID
    pub fn workflow() -> Self {
        Self::from_name("workflow.cim")
    }

    /// Knowledge context ID
    pub fn knowledge() -> Self {
        Self::from_name("knowledge.cim")
    }

    /// Content context ID
    pub fn content() -> Self {
        Self::from_name("content.cim")
    }
}

/// A node in a concept graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    /// Unique identifier for this node
    pub id: Uuid,

    /// Type of concept this node represents
    pub concept_type: String,

    /// Human-readable label
    pub label: String,

    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
}

impl ConceptNode {
    /// Create a new concept node
    pub fn new(concept_type: String, label: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            concept_type,
            label,
            properties: HashMap::new(),
        }
    }

    /// Add a property to the node
    pub fn with_property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }
}

/// An edge in a concept graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptEdge {
    /// Type of relationship
    pub relationship_type: String,

    /// Strength or weight of the relationship
    pub strength: f64,

    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
}

impl ConceptEdge {
    /// Create a new concept edge
    pub fn new(relationship_type: String, strength: f64) -> Self {
        Self {
            relationship_type,
            strength,
            properties: HashMap::new(),
        }
    }

    /// Add a property to the edge
    pub fn with_property(mut self, key: String, value: serde_json::Value) -> Self {
        self.properties.insert(key, value);
        self
    }
}

/// Type alias for the concept graph
pub type ConceptGraph = Graph<ConceptNode, ConceptEdge>;

/// A concept map stored in the Object Store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptMap {
    /// Unique identifier for this concept map
    pub id: ConceptMapId,

    /// The bounded context this concept belongs to
    pub context: ContextId,

    /// Content-addressed identifier for this version
    pub cid: Option<Cid>,

    /// The actual concept graph
    pub graph: ConceptGraph,

    /// Position in conceptual space
    pub position: ConceptualPoint,

    /// Quality values for each dimension
    pub qualities: HashMap<DimensionId, f64>,

    /// Events that created/modified this concept (CIDs of persisted events)
    pub event_history: Vec<Cid>,

    /// Count of transient events that affected this concept
    pub transient_event_count: u64,

    /// Timestamp of last modification
    pub last_modified: u64,
}

impl ConceptMap {
    /// Create a new concept map
    pub fn new(context: ContextId, position: ConceptualPoint) -> Self {
        Self {
            id: ConceptMapId::new(),
            context,
            cid: None,
            graph: ConceptGraph::new(),
            position,
            qualities: HashMap::new(),
            event_history: Vec::new(),
            transient_event_count: 0,
            last_modified: 0,
        }
    }

    /// Add a node to the concept graph
    pub fn add_node(&mut self, node: ConceptNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, edge: ConceptEdge) {
        self.graph.add_edge(source, target, edge);
    }

    /// Set a quality value
    pub fn set_quality(&mut self, dimension_id: DimensionId, value: f64) {
        self.qualities.insert(dimension_id, value);
    }

    /// Add an event to the history
    pub fn add_event(&mut self, event_cid: Cid) {
        self.event_history.push(event_cid);
    }

    /// Increment the transient event count
    pub fn increment_transient_events(&mut self) {
        self.transient_event_count += 1;
    }

    /// Get the root node of the graph (if any)
    pub fn root_node(&self) -> Option<NodeIndex> {
        // Find a node with no incoming edges
        self.graph.node_indices()
            .find(|&idx| self.graph.edges_directed(idx, petgraph::Direction::Incoming).count() == 0)
    }

    /// Get all nodes of a specific type
    pub fn nodes_by_type(&self, concept_type: &str) -> Vec<NodeIndex> {
        self.graph.node_indices()
            .filter(|&idx| {
                self.graph.node_weight(idx)
                    .map(|node| node.concept_type == concept_type)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Calculate the total strength of all edges
    pub fn total_edge_strength(&self) -> f64 {
        self.graph.edge_weights()
            .map(|edge| edge.strength)
            .sum()
    }
}

/// Builder for creating concept maps
pub struct ConceptMapBuilder {
    map: ConceptMap,
}

impl ConceptMapBuilder {
    /// Create a new builder
    pub fn new(context: ContextId, position: ConceptualPoint) -> Self {
        Self {
            map: ConceptMap::new(context, position),
        }
    }

    /// Add a node to the concept map
    pub fn with_node(mut self, node: ConceptNode) -> Self {
        self.map.add_node(node);
        self
    }

    /// Add an edge between nodes by their labels
    pub fn with_edge(mut self, source_label: &str, target_label: &str, edge: ConceptEdge) -> Self {
        // Find nodes by label
        let source_idx = self.map.graph.node_indices()
            .find(|&idx| {
                self.map.graph.node_weight(idx)
                    .map(|n| n.label == source_label)
                    .unwrap_or(false)
            });

        let target_idx = self.map.graph.node_indices()
            .find(|&idx| {
                self.map.graph.node_weight(idx)
                    .map(|n| n.label == target_label)
                    .unwrap_or(false)
            });

        if let (Some(source), Some(target)) = (source_idx, target_idx) {
            self.map.add_edge(source, target, edge);
        }

        self
    }

    /// Set a quality value
    pub fn with_quality(mut self, dimension_id: DimensionId, value: f64) -> Self {
        self.map.set_quality(dimension_id, value);
        self
    }

    /// Add an event to the history
    pub fn with_event(mut self, event_cid: Cid) -> Self {
        self.map.add_event(event_cid);
        self
    }

    /// Build the concept map
    pub fn build(self) -> ConceptMap {
        self.map
    }
}
