use super::{ExpressionOutcome, Risk};

/// The outcome of evaluating a crate against policy expressions.
#[derive(Debug, Clone)]
pub struct Appraisal {
    pub risk: Risk,
    pub expression_outcomes: Vec<ExpressionOutcome>,
    pub available_points: u32,
    pub awarded_points: u32,
    /// Raw weighted score storage. Use [`Self::weighted_score`] to account for
    /// required gates that skip weighted scoring.
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
            // Weighted scores are always non-negative. Negative zero preserves
            // numeric compatibility while recording that scoring was skipped.
            score: -0.0,
        }
    }

    #[must_use]
    pub const fn is_required_check_failure(&self) -> bool {
        self.score.is_sign_negative()
    }

    /// Returns the weighted score, or `None` when a required gate skipped scoring.
    #[must_use]
    pub const fn weighted_score(&self) -> Option<f64> {
        if self.is_required_check_failure() {
            None
        } else {
            Some(self.score)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::ExpressionDisposition;

    #[test]
    fn test_required_check_failure_records_explicit_state_without_changing_numeric_score() {
        let appraisal = Appraisal::required_check_failure(vec![ExpressionOutcome::new(
            "Required".into(),
            "Required policy".into(),
            ExpressionDisposition::False,
        )]);

        assert!(appraisal.is_required_check_failure());
        assert_eq!(appraisal.weighted_score(), None);
        assert_eq!(appraisal.score.to_bits(), (-0.0_f64).to_bits());
    }

    #[test]
    fn test_zero_point_appraisal_is_not_a_required_check_failure() {
        let appraisal = Appraisal::new(
            Risk::High,
            vec![ExpressionOutcome::new(
                "Weighted".into(),
                "Weighted policy".into(),
                ExpressionDisposition::False,
            )],
            0,
            0,
            0.0,
        );

        assert!(!appraisal.is_required_check_failure());
        assert_eq!(appraisal.weighted_score(), Some(0.0));
    }
}
