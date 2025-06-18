//! Example: Implementing Private Mortgage Lending Concepts in CIM's Conceptual Space
//!
//! This example demonstrates how to represent private mortgage lending concepts
//! (hard money loans, bridge loans, fix & flip, etc.) using topological conceptual spaces.

use cim_domain_conceptualspaces::{
    ConceptualSpace, ConceptualPoint, ConvexRegion, DimensionId,
    ConceptualMetric, QualityDimension, DimensionType, DistanceMetric,
    ContextId,
    CrossContextMorphism, MorphismType, ConceptId,
    ConceptualProjection, ConceptualChange,
};
use std::collections::HashMap;
use uuid::Uuid;

/// Quality dimensions specific to private mortgage lending
struct PrivateMortgageDimensions {
    // Asset-based dimensions (primary in private lending)
    ltv_ratio: DimensionId,
    property_quality: DimensionId,
    location_desirability: DimensionId,
    i: DimensionId,

    // Speed dimensions (key differentiator)
    funding_speed: DimensionId,
    decision_speed: DimensionId,
    documentation_flexibility: DimensionId,

    // Borrower dimensions
    borrower_experience: DimensionId,
    credit_relevance: DimensionId,
    liquidity_proof: DimensionId,

    // Deal structure dimensions
    rate_premium: DimensionId,
    points_charged: DimensionId,
    prepayment_flexibility: DimensionId,

    // Risk dimensions
    foreclosure_probability: DimensionId,
    renovation_risk: DimensionId,
    market_timing_risk: DimensionId,
}

impl PrivateMortgageDimensions {
    fn new() -> Self {
        Self {
            ltv_ratio: DimensionId::new(),
            property_quality: DimensionId::new(),
            location_desirability: DimensionId::new(),
            exit_strategy_clarity: DimensionId::new(),
            funding_speed: DimensionId::new(),
            decision_speed: DimensionId::new(),
            documentation_flexibility: DimensionId::new(),
            borrower_experience: DimensionId::new(),
            credit_relevance: DimensionId::new(),
            liquidity_proof: DimensionId::new(),
            rate_premium: DimensionId::new(),
            points_charged: DimensionId::new(),
            prepayment_flexibility: DimensionId::new(),
            foreclosure_probability: DimensionId::new(),
            renovation_risk: DimensionId::new(),
            market_timing_risk: DimensionId::new(),
        }
    }

    fn all_dimensions(&self) -> Vec<DimensionId> {
        vec![
            self.ltv_ratio,
            self.property_quality,
            self.location_desirability,
            self.exit_strategy_clarity,
            self.funding_speed,
            self.decision_speed,
            self.documentation_flexibility,
            self.borrower_experience,
            self.credit_relevance,
            self.liquidity_proof,
            self.rate_premium,
            self.points_charged,
            self.prepayment_flexibility,
            self.foreclosure_probability,
            self.renovation_risk,
            self.market_timing_risk,
        ]
    }

    fn create_dimension_map(&self) -> HashMap<DimensionId, usize> {
        self.all_dimensions()
            .into_iter()
            .enumerate()
            .map(|(idx, dim)| (dim, idx))
            .collect()
    }
}

/// Create quality dimension definitions for private mortgage lending
fn create_private_mortgage_dimensions(dims: &PrivateMortgageDimensions) -> Vec<QualityDimension> {
    vec![
        QualityDimension {
            id: dims.ltv_ratio,
            name: "LTV Ratio".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = 0% LTV, 1.0 = 100% LTV".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.property_quality,
            name: "Property Quality".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = distressed, 1.0 = prime condition".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.location_desirability,
            name: "Location Desirability".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = rural/declining, 1.0 = prime urban".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.exit_strategy_clarity,
            name: "Exit Strategy Clarity".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = unclear, 1.0 = guaranteed exit".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.funding_speed,
            name: "Funding Speed".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = 30+ days, 1.0 = same day".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.rate_premium,
            name: "Rate Premium".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = bank rate, 1.0 = maximum legal rate".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        QualityDimension {
            id: dims.renovation_risk,
            name: "Renovation Risk".to_string(),
            dimension_type: DimensionType::Continuous,
            range: 0.0..1.0,
            description: Some("0.0 = turnkey, 1.0 = complete gut rehab".to_string()),
            context: Some("PrivateMortgageLending".to_string()),
        },
        // ... additional dimensions
    ]
}

/// Create a hard money loan prototype
fn create_hard_money_prototype(dims: &PrivateMortgageDimensions) -> ConceptualPoint {
    let values = vec![
        0.7,  // ltv_ratio: 70% typical
        0.4,  // property_quality: often distressed
        0.6,  // location_desirability: varies
        0.8,  // exit_strategy_clarity: clear flip/refinance plan
        0.9,  // funding_speed: very fast (days)
        0.95, // decision_speed: hours to days
        0.8,  // documentation_flexibility: asset-focused
        0.7,  // borrower_experience: prefer experienced
        0.2,  // credit_relevance: low importance
        0.6,  // liquidity_proof: moderate reserves needed
        0.8,  // rate_premium: 12-15% typical
        0.7,  // points_charged: 2-4 points
        0.9,  // prepayment_flexibility: no penalties
        0.3,  // foreclosure_probability: moderate
        0.8,  // renovation_risk: high (fix & flip)
        0.5,  // market_timing_risk: 6-12 month exposure
    ];

    ConceptualPoint::new(values, dims.create_dimension_map())
}

/// Create a bridge loan prototype
fn create_bridge_loan_prototype(dims: &PrivateMortgageDimensions) -> ConceptualPoint {
    let values = vec![
        0.75, // ltv_ratio: 75% typical
        0.7,  // property_quality: better condition
        0.7,  // location_desirability: better locations
        0.9,  // exit_strategy_clarity: clear transition plan
        0.7,  // funding_speed: fast but not urgent
        0.8,  // decision_speed: days to week
        0.6,  // documentation_flexibility: more docs than hard money
        0.8,  // borrower_experience: experienced preferred
        0.4,  // credit_relevance: some consideration
        0.7,  // liquidity_proof: stronger reserves
        0.6,  // rate_premium: 8-12% typical
        0.5,  // points_charged: 1-2 points
        0.7,  // prepayment_flexibility: some restrictions
        0.2,  // foreclosure_probability: lower risk
        0.3,  // renovation_risk: minimal renovations
        0.6,  // market_timing_risk: 12-36 month exposure
    ];

    ConceptualPoint::new(values, dims.create_dimension_map())
}

/// Create a construction loan prototype
fn create_construction_loan_prototype(dims: &PrivateMortgageDimensions) -> ConceptualPoint {
    let values = vec![
        0.65, // ltv_ratio: 65% of completed value
        0.2,  // property_quality: raw land/teardown
        0.8,  // location_desirability: prime locations for new construction
        0.7,  // exit_strategy_clarity: sale or perm financing
        0.5,  // funding_speed: slower due to complexity
        0.4,  // decision_speed: detailed underwriting
        0.3,  // documentation_flexibility: extensive docs required
        0.9,  // borrower_experience: must be experienced builder
        0.5,  // credit_relevance: moderate importance
        0.8,  // liquidity_proof: strong reserves for overruns
        0.7,  // rate_premium: 10-14% typical
        0.6,  // points_charged: 2-3 points
        0.5,  // prepayment_flexibility: tied to completion
        0.4,  // foreclosure_probability: higher risk
        0.95, // renovation_risk: maximum (ground-up)
        0.7,  // market_timing_risk: 12-18 month projects
    ];

    ConceptualPoint::new(values, dims.create_dimension_map())
}

/// Domain events specific to private mortgage lending
#[derive(Debug, Clone)]
enum PrivateMortgageEvent {
    LoanOriginated {
        loan_id: Uuid,
        loan_type: String,
        property_address: String,
        ltv: f64,
        rate: f64,
        term_months: u32,
    },
    DrawRequested {
        loan_id: Uuid,
        draw_number: u32,
        amount: f64,
        construction_progress: f64,
    },
    PropertyInspected {
        loan_id: Uuid,
        inspection_result: String,
        value_change: f64,
    },
    ExitStrategyUpdated {
        loan_id: Uuid,
        old_strategy: String,
        new_strategy: String,
        confidence: f64,
    },
    PayoffReceived {
        loan_id: Uuid,
        payoff_amount: f64,
        days_outstanding: u32,
    },
    DefaultNoticeIssued {
        loan_id: Uuid,
        days_delinquent: u32,
        outstanding_balance: f64,
    },
}

/// Project private mortgage events into conceptual space changes
impl ConceptualProjection for PrivateMortgageEvent {
    fn project(&self) -> Vec<ConceptualChange> {
        match self {
            PrivateMortgageEvent::PropertyInspected { loan_id, value_change, .. } => {
                // Property value changes require removing old concept and adding new one
                let concept_id = ConceptId::from_uuid(*loan_id);
                let new_position = self.calculate_new_position_after_inspection(*value_change);

                vec![
                    ConceptualChange::RemoveConcept { concept_id },
                    ConceptualChange::AddConcept {
                        concept_id,
                        concept_type: "PrivateMortgage".to_string(),
                        position: new_position,
                        qualities: self.concept_qualities(),
                    }
                ]
            }
            PrivateMortgageEvent::ExitStrategyUpdated { loan_id, confidence, .. } => {
                // Exit strategy change requires concept replacement
                let concept_id = ConceptId::from_uuid(*loan_id);
                let new_position = self.calculate_position_with_exit_confidence(*confidence);

                vec![
                    ConceptualChange::RemoveConcept { concept_id },
                    ConceptualChange::AddConcept {
                        concept_id,
                        concept_type: "PrivateMortgage".to_string(),
                        position: new_position,
                        qualities: self.concept_qualities(),
                    }
                ]
            }
            PrivateMortgageEvent::DefaultNoticeIssued { loan_id, days_delinquent, .. } => {
                // Default moves loan to distressed region - remove and re-add
                let concept_id = ConceptId::from_uuid(*loan_id);
                let distressed_position = self.calculate_default_position(*days_delinquent);

                vec![
                    ConceptualChange::RemoveConcept { concept_id },
                    ConceptualChange::AddConcept {
                        concept_id,
                        concept_type: "DistressedMortgage".to_string(),
                        position: distressed_position,
                        qualities: self.concept_qualities(),
                    }
                ]
            }
            _ => vec![],
        }
    }

    fn affected_concepts(&self) -> Vec<ConceptId> {
        match self {
            PrivateMortgageEvent::LoanOriginated { loan_id, .. } |
            PrivateMortgageEvent::DrawRequested { loan_id, .. } |
            PrivateMortgageEvent::PropertyInspected { loan_id, .. } |
            PrivateMortgageEvent::ExitStrategyUpdated { loan_id, .. } |
            PrivateMortgageEvent::PayoffReceived { loan_id, .. } |
            PrivateMortgageEvent::DefaultNoticeIssued { loan_id, .. } => {
                vec![ConceptId::from_uuid(*loan_id)]
            }
        }
    }

    fn concept_qualities(&self) -> HashMap<DimensionId, f64> {
        HashMap::new() // Simplified for example
    }

    fn event_type(&self) -> &str {
        match self {
            PrivateMortgageEvent::LoanOriginated { .. } => "private_mortgage.originated",
            PrivateMortgageEvent::DrawRequested { .. } => "private_mortgage.draw.requested",
            PrivateMortgageEvent::PropertyInspected { .. } => "private_mortgage.property.inspected",
            PrivateMortgageEvent::ExitStrategyUpdated { .. } => "private_mortgage.exit.updated",
            PrivateMortgageEvent::PayoffReceived { .. } => "private_mortgage.payoff.received",
            PrivateMortgageEvent::DefaultNoticeIssued { .. } => "private_mortgage.default.notice",
        }
    }
}

impl PrivateMortgageEvent {
    fn calculate_default_position(&self, days_delinquent: u32) -> ConceptualPoint {
        // Move to distressed region based on delinquency
        let risk_factor = (days_delinquent as f64 / 90.0).min(1.0);
        let values = vec![
            0.85,        // ltv_ratio: likely underwater
            0.2,         // property_quality: deteriorating
            0.5,         // location_desirability: unchanged
            0.1,         // exit_strategy_clarity: very unclear
            0.1,         // funding_speed: n/a
            0.1,         // decision_speed: n/a
            0.1,         // documentation_flexibility: n/a
            0.5,         // borrower_experience: unchanged
            0.0,         // credit_relevance: damaged
            0.0,         // liquidity_proof: exhausted
            1.0,         // rate_premium: default rate
            0.0,         // points_charged: n/a
            0.0,         // prepayment_flexibility: n/a
            risk_factor, // foreclosure_probability: high
            0.5,         // renovation_risk: unknown
            0.9,         // market_timing_risk: forced sale
        ];

        ConceptualPoint::new(values, HashMap::new()) // Simplified
    }

    fn calculate_new_position_after_inspection(&self, value_change: f64) -> ConceptualPoint {
        // Recalculate position based on property value change
        let ltv_improvement = (value_change / 100.0).max(-0.5).min(0.5);
        let values = vec![
            0.7 - ltv_improvement * 0.2,  // ltv_ratio: improves with value increase
            0.4 + ltv_improvement * 0.3,  // property_quality: better if value up
            0.6,                           // location_desirability: unchanged
            0.8,                           // exit_strategy_clarity: unchanged
            0.9,                           // funding_speed: unchanged
            0.95,                          // decision_speed: unchanged
            0.8,                           // documentation_flexibility: unchanged
            0.7,                           // borrower_experience: unchanged
            0.2,                           // credit_relevance: unchanged
            0.6,                           // liquidity_proof: unchanged
            0.8,                           // rate_premium: unchanged
            0.7,                           // points_charged: unchanged
            0.9,                           // prepayment_flexibility: unchanged
            0.3 - ltv_improvement * 0.2,  // foreclosure_probability: lower if value up
            0.8,                           // renovation_risk: unchanged
            0.5,                           // market_timing_risk: unchanged
        ];

        ConceptualPoint::new(values, HashMap::new())
    }

    fn calculate_position_with_exit_confidence(&self, confidence: f64) -> ConceptualPoint {
        // Adjust position based on exit strategy confidence
        let values = vec![
            0.7,                          // ltv_ratio: unchanged
            0.4,                          // property_quality: unchanged
            0.6,                          // location_desirability: unchanged
            0.8 * confidence,             // exit_strategy_clarity: scales with confidence
            0.9,                          // funding_speed: unchanged
            0.95,                         // decision_speed: unchanged
            0.8,                          // documentation_flexibility: unchanged
            0.7,                          // borrower_experience: unchanged
            0.2,                          // credit_relevance: unchanged
            0.6,                          // liquidity_proof: unchanged
            0.8,                          // rate_premium: unchanged
            0.7,                          // points_charged: unchanged
            0.9,                          // prepayment_flexibility: unchanged
            0.3 * (2.0 - confidence),     // foreclosure_probability: inverse of confidence
            0.8,                          // renovation_risk: unchanged
            0.5 * (2.0 - confidence),     // market_timing_risk: lower with better exit
        ];

        ConceptualPoint::new(values, HashMap::new())
    }
}

/// Cross-context morphisms for private mortgage lending
fn create_private_mortgage_morphisms() -> Vec<CrossContextMorphism> {
    let origination_context = ContextId::new();
    let servicing_context = ContextId::new();
    let asset_management_context = ContextId::new();
    let investor_context = ContextId::new();

    vec![
        // Loan origination to servicing handoff
        CrossContextMorphism::new(
            (origination_context, ConceptId::new()),
            (servicing_context, ConceptId::new()),
            MorphismType::StateMapping,
            0.95, // High fidelity transfer
        ),

        // Servicing to asset management (for workouts)
        CrossContextMorphism::new(
            (servicing_context, ConceptId::new()),
            (asset_management_context, ConceptId::new()),
            MorphismType::PolicyApplication,
            0.85, // Some context-specific info
        ),

        // Asset performance to investor reporting
        CrossContextMorphism::new(
            (asset_management_context, ConceptId::new()),
            (investor_context, ConceptId::new()),
            MorphismType::SemanticLink,
            0.7, // Summarized for investors
        ),
    ]
}

fn main() {
    println!("=== Private Mortgage Lending Concepts in CIM ===\n");

    // 1. Create private mortgage dimensions
    let dims = PrivateMortgageDimensions::new();
    let quality_dims = create_private_mortgage_dimensions(&dims);

    println!("Created {} quality dimensions for private mortgages", quality_dims.len());

    // 2. Create conceptual space for private lending
    let metric = ConceptualMetric::uniform(16, 2.0); // Euclidean metric
    let mut lending_space = ConceptualSpace::new(
        "Private Mortgage Lending Space".to_string(),
        dims.all_dimensions(),
        metric,
    );

    // 3. Create loan type prototypes
    let hard_money_prototype = create_hard_money_prototype(&dims);
    let bridge_loan_prototype = create_bridge_loan_prototype(&dims);
    let construction_prototype = create_construction_loan_prototype(&dims);

    // Add prototypes to space
    let hard_money_id = lending_space.add_point(hard_money_prototype.clone()).unwrap();
    let bridge_id = lending_space.add_point(bridge_loan_prototype.clone()).unwrap();
    let construction_id = lending_space.add_point(construction_prototype.clone()).unwrap();

    println!("\nAdded private mortgage prototypes to conceptual space");

    // 4. Create regions for loan types
    let mut hard_money_region = ConvexRegion::from_prototype(hard_money_prototype.clone());
    hard_money_region.name = Some("Hard Money Loans".to_string());

    let mut bridge_region = ConvexRegion::from_prototype(bridge_loan_prototype.clone());
    bridge_region.name = Some("Bridge Loans".to_string());

    let mut construction_region = ConvexRegion::from_prototype(construction_prototype.clone());
    construction_region.name = Some("Construction Loans".to_string());

    lending_space.add_region(hard_money_region).unwrap();
    lending_space.add_region(bridge_region).unwrap();
    lending_space.add_region(construction_region).unwrap();

    println!("Created convex regions for private mortgage types");

    // 5. Calculate similarities between loan types
    println!("\n=== Loan Type Similarities ===");

    let hm_to_bridge = lending_space.metric.distance(
        &hard_money_prototype,
        &bridge_loan_prototype
    ).unwrap();

    let hm_to_construction = lending_space.metric.distance(
        &hard_money_prototype,
        &construction_prototype
    ).unwrap();

    let bridge_to_construction = lending_space.metric.distance(
        &bridge_loan_prototype,
        &construction_prototype
    ).unwrap();

    println!("Hard Money ↔ Bridge: {:.1}% similar", (1.0 - hm_to_bridge.min(1.0)) * 100.0);
    println!("Hard Money ↔ Construction: {:.1}% similar", (1.0 - hm_to_construction.min(1.0)) * 100.0);
    println!("Bridge ↔ Construction: {:.1}% similar", (1.0 - bridge_to_construction.min(1.0)) * 100.0);

    // 6. Process private mortgage events
    println!("\n=== Processing Private Mortgage Events ===");

    let loan_id = Uuid::new_v4();
    let events = vec![
        PrivateMortgageEvent::LoanOriginated {
            loan_id,
            loan_type: "Hard Money".to_string(),
            property_address: "123 Main St, Los Angeles, CA".to_string(),
            ltv: 0.68,
            rate: 12.5,
            term_months: 12,
        },
        PrivateMortgageEvent::PropertyInspected {
            loan_id,
            inspection_result: "Renovation 50% complete".to_string(),
            value_change: 15.0, // 15% increase
        },
        PrivateMortgageEvent::ExitStrategyUpdated {
            loan_id,
            old_strategy: "Flip to retail buyer".to_string(),
            new_strategy: "Refinance to conventional".to_string(),
            confidence: 0.85,
        },
    ];

    for event in &events {
        println!("\nEvent: {}", event.event_type());
        let changes = event.project();
        for change in changes {
            match change {
                ConceptualChange::RemoveConcept { .. } => {
                    println!("  → Removed old loan concept");
                }
                ConceptualChange::AddConcept { concept_type, .. } => {
                    println!("  → Added new loan concept as: {}", concept_type);
                }
                _ => {}
            }
        }
    }

    // 7. Find best loan type for new scenario
    println!("\n=== Loan Type Recommendation ===");

    // New loan scenario: Quick funding needed, decent property, clear exit
    let new_scenario = ConceptualPoint::new(
        vec![
            0.72, // ltv_ratio: 72%
            0.6,  // property_quality: decent
            0.7,  // location_desirability: good area
            0.9,  // exit_strategy_clarity: very clear
            0.85, // funding_speed: need fast
            0.9,  // decision_speed: need quick decision
            0.7,  // documentation_flexibility: some flexibility needed
            0.6,  // borrower_experience: moderate
            0.3,  // credit_relevance: poor credit
            0.5,  // liquidity_proof: some reserves
            0.7,  // rate_premium: willing to pay
            0.6,  // points_charged: acceptable
            0.8,  // prepayment_flexibility: want flexibility
            0.25, // foreclosure_probability: low risk
            0.4,  // renovation_risk: light rehab
            0.4,  // market_timing_risk: 6-9 months
        ],
        dims.create_dimension_map(),
    );

    let recommendations = lending_space.k_nearest_neighbors(&new_scenario, 3).unwrap();

    println!("\nBest loan types for scenario:");
    for (id, distance) in recommendations {
        let loan_type = if *id == hard_money_id {
            "Hard Money Loan"
        } else if *id == bridge_id {
            "Bridge Loan"
        } else {
            "Construction Loan"
        };

        println!("  {} - Match: {:.1}%",
            loan_type, (1.0 - distance.min(1.0)) * 100.0);
    }

    println!("\n=== Market Evolution ===");
    println!("As private lending evolves, new dimensions emerge:");
    println!("  - Cryptocurrency collateral acceptance");
    println!("  - ESG compliance requirements");
    println!("  - Digital property tokenization");
    println!("  - Automated valuation model (AVM) reliance");
    println!("\nThe conceptual space adapts to represent these new aspects!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_mortgage_dimensions() {
        let dims = PrivateMortgageDimensions::new();
        assert_eq!(dims.all_dimensions().len(), 16);
    }

    #[test]
    fn test_loan_type_differentiation() {
        let dims = PrivateMortgageDimensions::new();
        let hard_money = create_hard_money_prototype(&dims);
        let construction = create_construction_loan_prototype(&dims);

        // Hard money and construction loans should be quite different
        let distance = hard_money.weighted_distance(
            &construction,
            &vec![1.0; 16],
            2.0
        ).unwrap();

        assert!(distance > 0.4); // Significantly different due to renovation risk
    }
}
