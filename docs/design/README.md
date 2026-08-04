# cargo-aprz Design

## Appraisal output

`cargo-aprz` evaluates each crate in two stages:

1. Required `high_risk` checks are evaluated first. A false result or evaluation
   failure immediately makes the crate high risk, and the weighted score is not
   calculated.
2. If all required checks pass, weighted `eval` checks produce a score and the
   configured thresholds determine the risk level.

Reports distinguish required-check policy failures, required checks that could
not be evaluated, and weighted scores. Failed and inconclusive expression
results include both the configured check name and description so users can
understand what the policy requires. Passing results remain name-only to keep
output concise.

Required-gate state is encoded separately from threshold and point totals while
preserving the existing public `Appraisal` struct shape. Production consumers use
`weighted_score`, which returns no score when a required gate skipped weighted
evaluation; raw score storage is retained only for struct-literal compatibility.

CSV reports neutralize textual metric cells whose first non-whitespace character
is a spreadsheet formula marker (`=`, `+`, `-`, or `@`) by prefixing an
apostrophe. This includes formulas hidden behind spaces, tabs, or line breaks.
Numeric metric values remain numeric so spreadsheet consumers can continue to
sort and calculate with them. Excel reports use string cells for textual values,
which prevents them from being interpreted as formulas.

JSON preserves the legacy `result` and `reasons` fields, including the
`failure to evaluate` suffix for inconclusive reasons, while exposing structured
risk, score, point, required-check, and per-expression outcome fields. Score and
point fields are null when a required gate prevents weighted scoring. Consumers
should use the structured fields rather than parsing display strings.

When `--error-if-high-risk` or `--error-if-medium-risk` rejects a run, the final
error lists up to 20 non-allowed crates that caused the rejection and up to 10
appraisal outcomes per crate, with omitted-item counts and instructions for
requesting a complete console or JSON report. Required-gate rejections distinguish
policy failures from inconclusive evaluations, identify the crate as high risk,
and state that weighted scoring was not calculated. When console appraisal reasons
are not rendered, including partial `--console` modes, the error includes
descriptions and evaluation-failure reasons so it remains actionable as the
command's only diagnostic. When the console already rendered appraisal reasons,
the error remains concise to avoid repeating those details. Score-based
rejections use a consistent risk-and-score format and include bounded non-passing
weighted outcomes when the console omitted them. The error directs users to
remediate, upgrade, or replace the dependency.
If a temporary policy exception is appropriate, users can add an exact crate
version to `[[allow_list]]`; allowed crates remain visible in reports but do not
fail the command.
