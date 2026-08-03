use super::{ExpressionOutcome, Risk};

/// The outcome of evaluating a crate against policy expressions.
#[derive(Debug, Clone)]
pub struct Appraisal {
    pub risk: Risk,
    pub required_check_failure: bool,
    pub expression_outcomes: Vec<ExpressionOutcome>,
    pub available_points: u32,
    pub awarded_points: u32,
    pub score: f64,
}

impl Appraisal {
    #[must_use]
    pub const fn new(
        risk: Risk,
        expression_outcomes: Vec<ExpressionOutcome>,
        available_points: u32,
        awarded_points: u32,
        score: f64,
    ) -> Self {
        Self {
            risk,
            required_check_failure: false,
            expression_outcomes,
            available_points,
            awarded_points,
            score,
        }
    }

    #[must_use]
    pub const fn required_check_failure(
        expression_outcomes: Vec<ExpressionOutcome>,
    ) -> Self {
        Self {
            risk: Risk::High,
            required_check_failure: true,
            expression_outcomes,
            available_points: 0,
            awarded_points: 0,
            score: 0.0,
        }
    }
}
