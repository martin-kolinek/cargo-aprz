use super::{ExpressionOutcome, Risk};

/// The outcome of evaluating a crate against policy expressions.
#[derive(Debug, Clone)]
pub struct Appraisal {
    pub risk: Risk,
    pub expression_outcomes: Vec<ExpressionOutcome>,
    pub available_points: u32,
    pub awarded_points: u32,
    /// Raw weighted score storage. Use [`Self::weighted_score`] to account for
    /// evaluation states that do not produce a score.
    pub score: f64,
}

impl Appraisal {
    const REQUIRED_CHECK_FAILURE_SCORE: f64 = -0.0;
    const WEIGHTED_EVALUATION_FAILURE_SCORE: f64 = -1.0;

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
            score: Self::REQUIRED_CHECK_FAILURE_SCORE,
        }
    }

    #[must_use]
    pub fn weighted_evaluation_failure(expression_outcomes: Vec<ExpressionOutcome>) -> Self {
        debug_assert!(
            expression_outcomes.iter().any(|outcome| {
                matches!(outcome.disposition, super::ExpressionDisposition::Failed(_))
            }),
            "weighted-evaluation failures must contain an inconclusive outcome"
        );
        Self {
            risk: Risk::High,
            expression_outcomes,
            available_points: 0,
            awarded_points: 0,
            score: Self::WEIGHTED_EVALUATION_FAILURE_SCORE,
        }
    }

    #[must_use]
    pub const fn is_required_check_failure(&self) -> bool {
        self.score.to_bits() == Self::REQUIRED_CHECK_FAILURE_SCORE.to_bits()
    }

    #[must_use]
    pub const fn is_weighted_evaluation_failure(&self) -> bool {
        self.score.to_bits() == Self::WEIGHTED_EVALUATION_FAILURE_SCORE.to_bits()
    }

    /// Returns the weighted score, or `None` when evaluation did not produce one.
    #[must_use]
    pub const fn weighted_score(&self) -> Option<f64> {
        if self.is_required_check_failure() || self.is_weighted_evaluation_failure() {
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

    #[test]
    fn test_weighted_evaluation_failure_records_distinct_skipped_score_state() {
        let appraisal = Appraisal::weighted_evaluation_failure(vec![ExpressionOutcome::new(
            "Weighted".into(),
            "Weighted policy".into(),
            ExpressionDisposition::Failed("unavailable".into()),
        )]);

        assert!(!appraisal.is_required_check_failure());
        assert!(appraisal.is_weighted_evaluation_failure());
        assert_eq!(appraisal.weighted_score(), None);
    }
}
