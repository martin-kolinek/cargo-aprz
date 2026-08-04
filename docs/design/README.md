# cargo-aprz Design

## Appraisal output

`cargo-aprz` evaluates each crate in two stages:

1. Required `high_risk` checks are evaluated first. A false result or evaluation
   failure immediately makes the crate high risk, and the weighted score is not
   calculated.
2. If all required checks pass, weighted `eval` checks produce a score and the
   configured thresholds determine the risk level.

Reports distinguish a failed required check from a zero-point weighted score.
Failed and inconclusive expression results include both the configured check
name and description so users can understand what the policy requires. Passing
results remain name-only to keep output concise.

CSV reports neutralize text cells whose first non-whitespace character is a
spreadsheet formula marker (`=`, `+`, `-`, or `@`) by prefixing an apostrophe.
This includes formulas hidden behind spaces, tabs, or line breaks. Numeric
metric values remain numeric so spreadsheet consumers can continue to sort and
calculate with them.

When `--error-if-high-risk` or `--error-if-medium-risk` rejects a run, the final
error lists every non-allowed crate that caused the rejection. Required-gate
rejections include the failed check names; score-based rejections include the
score rather than every weighted expression that contributed to it. The error
directs users to remediate, upgrade, or replace the dependency.
If a temporary policy exception is appropriate, users can add an exact crate
version to `[[allow_list]]`; allowed crates remain visible in reports but do not
fail the command.
