use super::{ExpressionOutcome, Risk};

/// The outcome of evaluating a crate against policy expressions.
#[derive(Debug, Clone)]
pub struct Appraisal {
    pub risk: Risk,
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
            expression_outcomes,
            available_points,
            awarded_points,
            score,
        }
    }

    #[must_use]
    pub fn required_check_failure(expression_outcomes: Vec<ExpressionOutcome>) -> Self {
        debug_assert!(
            expression_outcomes
                .iter()
                .any(|outcome| !matches!(outcome.disposition, super::ExpressionDisposition::True)),
            "required-check appraisals must contain a failed or inconclusive outcome"
        );
        Self {
            risk: Risk::High,
            expression_outcomes,
            available_points: 0,
            awarded_points: 0,
            score: 0.0,
        }
    }

    #[must_use]
    pub fn is_required_check_failure(&self) -> bool {
        self.risk == Risk::High
            && self.available_points == 0
            && self.awarded_points == 0
            && self
                .expression_outcomes
                .iter()
                .any(|outcome| !matches!(outcome.disposition, super::ExpressionDisposition::True))
    }
}
