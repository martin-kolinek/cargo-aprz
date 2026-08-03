# cargo-aprz Design

## Appraisal output

`cargo-aprz` evaluates each crate in two stages:

1. Required `high_risk` checks are evaluated first. A false result or evaluation
   failure immediately makes the crate high risk, and the weighted score is not
   calculated.
2. If all required checks pass, weighted `eval` checks produce a score and the
   configured thresholds determine the risk level.

Reports distinguish a failed required check from a zero-point weighted score.
Expression results include both the configured check name and description so
users can understand what the policy requires.

CSV reports neutralize text cells beginning with spreadsheet formula characters
(`=`, `+`, `-`, or `@`) by prefixing an apostrophe. Numeric metric values remain
numeric so spreadsheet consumers can continue to sort and calculate with them.

When `--error-if-high-risk` or `--error-if-medium-risk` rejects a run, the final
error lists every non-allowed crate that caused the rejection and its failed
checks. The error directs users to remediate, upgrade, or replace the dependency.
If a temporary policy exception is appropriate, users can add an exact crate
version to `[[allow_list]]`; allowed crates remain visible in reports but do not
fail the command.
