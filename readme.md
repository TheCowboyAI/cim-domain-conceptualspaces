# CIM Domain: Conceptual Spaces

## Overview

The Conceptual Spaces domain implements Gärdenfors' theory of conceptual spaces for semantic reasoning, similarity measurement, and knowledge representation in the CIM system. It provides geometric representations of concepts, enabling AI-ready semantic understanding and reasoning about relationships between entities.

## Key Features

- **Geometric Concept Representation**: Maps concepts to points in multi-dimensional spaces
- **Similarity Measurement**: Calculate semantic distance between concepts
- **Region Formation**: Create convex regions representing natural categories
- **Quality Dimensions**: Define interpretable dimensions for concept spaces
- **Voronoi Tessellation**: Partition space based on concept prototypes
- **Dynamic Learning**: Adapt spaces based on new examples and feedback

## Architecture

### Domain Structure
- **Aggregates**: `ConceptualSpace`, `ConceptSpace` 
- **Value Objects**: `ConceptualPoint`, `QualityDimension`, `ConvexRegion`, `Similarity`
- **Commands**: `CreateSpace`, `AddConcept`, `AddRegion`, `UpdateWeights`
- **Events**: `SpaceCreated`, `ConceptAdded`, `RegionFormed`, `WeightsUpdated`
- **Queries**: `FindSimilarConcepts`, `GetConceptPosition`, `FindRegionForPoint`

### Integration Points
- **Graph Domain**: Visualize conceptual relationships as graphs
- **Agent Domain**: Enable semantic reasoning for AI agents
- **Workflow Domain**: Use similarity for intelligent routing
- **Identity Domain**: Conceptual understanding of entity relationships

## Usage Example

```rust
use cim_domain_conceptualspaces::{
    commands::{CreateSpace, AddConcept},
    value_objects::{QualityDimension, ConceptualPoint},
};

// Create a color space
let create_space = CreateSpace {
    space_id: SpaceId::new(),
    name: "ColorSpace".to_string(),
    dimensions: vec![
        QualityDimension::circular("hue", 0.0, 360.0),
        QualityDimension::linear("saturation", 0.0, 1.0),
        QualityDimension::linear("lightness", 0.0, 1.0),
    ],
};

// Add a concept
let add_concept = AddConcept {
    space_id,
    concept_id: ConceptId::new(),
    name: "Red".to_string(),
    position: ConceptualPoint::new(vec![0.0, 0.8, 0.5]), // HSL values
};
```

## Testing

Run domain tests:
```bash
cargo test -p cim-domain-conceptualspaces
```

## Documentation

- [User Stories](doc/user-stories.md) - Business requirements and use cases
- [API Documentation](doc/api.md) - Technical API reference
- [Category Theory](src/category_theory/) - Mathematical foundations

## Contributing

See the main project [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines. 