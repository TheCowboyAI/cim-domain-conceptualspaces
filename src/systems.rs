//! ECS Systems for Conceptual Spaces domain
//!
//! These systems manage concept graph entities and their markers

use bevy::prelude::*;
use cim_domain::identifiers::markers::ConceptGraphMarker as DomainConceptGraphMarker;

// Define the ConceptGraph marker component
#[derive(Component, Debug, Clone, Copy)]
pub struct ConceptGraphMarker(pub DomainConceptGraphMarker);

// Component to identify concept graph entities
#[derive(Component)]
pub struct ConceptGraphEntity {
    pub id: uuid::Uuid,
    pub name: String,
}

/// System that spawns concept graph entities with markers
pub fn spawn_concept_graph_system(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnConceptGraphEvent>,
) {
    for event in spawn_events.read() {
        // Spawn the concept graph entity with marker
        commands.spawn((
            ConceptGraphEntity {
                id: uuid::Uuid::new_v4(),
                name: event.name.clone(),
            },
            ConceptGraphMarker(DomainConceptGraphMarker),
        ));
        
        tracing::info!("Spawned concept graph entity: {}", event.name);
    }
}

/// Event to trigger concept graph spawning
#[derive(Event)]
pub struct SpawnConceptGraphEvent {
    pub name: String,
}

/// System that adds markers to existing concept graph entities
pub fn add_concept_graph_markers_system(
    mut commands: Commands,
    concept_graphs: Query<Entity, (With<ConceptGraphEntity>, Without<ConceptGraphMarker>)>,
) {
    for entity in concept_graphs.iter() {
        commands.entity(entity).insert(ConceptGraphMarker(DomainConceptGraphMarker));
    }
}

/// Example query using the concept graph marker
#[allow(dead_code)]
pub fn query_concept_graphs(
    graphs: Query<(&ConceptGraphEntity, Entity), With<ConceptGraphMarker>>,
) {
    for (graph, entity) in graphs.iter() {
        tracing::debug!(
            "Concept graph entity {:?} - ID: {}, Name: {}",
            entity,
            graph.id,
            graph.name
        );
    }
}

/// System that uses concept graph markers for processing
#[allow(dead_code)]
pub fn process_concept_graphs_system(
    graphs: Query<&ConceptGraphEntity, With<ConceptGraphMarker>>,
) {
    let graph_count = graphs.iter().count();
    tracing::debug!("Processing {} concept graphs", graph_count);
    
    for graph in graphs.iter() {
        // Process concept graph logic here
        tracing::trace!("Processing concept graph: {}", graph.name);
    }
}