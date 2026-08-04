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

CSV reports neutralize textual metric cells whose first non-whitespace character
is a spreadsheet formula marker (`=`, `+`, `-`, or `@`) by prefixing an
apostrophe. This includes formulas hidden behind spaces, tabs, or line breaks.
Numeric metric values remain numeric so spreadsheet consumers can continue to
sort and calculate with them. Excel reports use string cells for textual values,
which prevents them from being interpreted as formulas.

JSON preserves the legacy `result` and name-only `reasons` fields while exposing
structured risk, score, point, required-check, and per-expression outcome fields.
Consumers should use the structured fields rather than parsing display strings.

When `--error-if-high-risk` or `--error-if-medium-risk` rejects a run, the final
error lists up to 20 non-allowed crates that caused the rejection and up to 10
blocking required checks per crate, with omitted-item counts. Required-gate
rejections distinguish policy failures from inconclusive evaluations.
When console output is suppressed, the error includes descriptions and
evaluation-failure reasons so it remains actionable as the command's only
terminal output. When console output is present, the error remains concise to
avoid repeating those details. Score-based rejections use a consistent
risk-and-score format rather than listing every weighted expression. The error
directs users to remediate, upgrade, or replace the dependency.
If a temporary policy exception is appropriate, users can add an exact crate
version to `[[allow_list]]`; allowed crates remain visible in reports but do not
fail the command.
