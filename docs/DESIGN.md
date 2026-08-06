# cargo-aprz Architecture

## Overview

`cargo-aprz` turns crate identities into appraisals through a staged pipeline:

```text
crate selection
    -> fact collection
    -> metric extraction
    -> policy evaluation
    -> report rendering and optional rejection
```

The command layer selects explicit crates or resolves a Cargo dependency graph,
loads configuration, and coordinates the remaining stages. Each later stage
operates on typed data produced by the previous stage rather than reading source
systems directly.

## Fact collection

Fact providers retrieve independent views of each crate from crates.io, source
hosting services, the RustSec advisory database, docs.rs, coverage services, and
local source analysis. Providers return either typed data or an explicit
unavailable result so downstream evaluation can distinguish missing information
from a legitimate zero value.

Collection is asynchronous where providers can run independently. Source-code
analysis is grouped by repository so crates from the same repository share the
clone and repository-level work.

## Cache storage

Provider data is stored beneath a platform-specific cache root, partitioned by
source:

```text
cache root/
    crates/
    hosting/
    codebase/
    coverage/
    advisories/
    docs/
```

Most provider entries are MessagePack files containing a timestamp and either
typed data or a negative-cache result. Each provider has its own configurable
time-to-live. Expired, corrupt, or explicitly ignored entries are treated as
cache misses. Repository clones and the RustSec database are kept in their
provider partitions alongside synchronization metadata.

The process takes an advisory lock on the cache root before collection so two
instances do not concurrently mutate shared cache state.

## Metrics

Collected facts are normalized into named, typed metrics. Metric values may be
unsigned integers, floating-point values, booleans, strings, timestamps, or
lists. Policy expressions and report generators consume this common metric
representation, keeping source-specific response formats out of those stages.

Unavailable provider data normally becomes a missing or null metric. Expression
evaluation can therefore report that a policy was inconclusive instead of
silently converting unavailable information into a passing value.

## Policy evaluation

Each crate is evaluated in two phases:

1. Required `high_risk` expressions run first. A false or inconclusive result
   immediately produces a high-risk appraisal without a weighted score.
2. If the required phase passes, weighted `eval` expressions contribute awarded
   and available points. The percentage score is mapped to low, medium, or high
   risk using configured thresholds.

An expression that cannot be evaluated produces an inconclusive outcome and does
not contribute points. If positive-weight expressions are configured but every
one is inconclusive, evaluation fails closed as high risk without a score. An
empty weighted policy, or a policy containing only zero-weight expressions,
remains low risk with the neutral score of 100.

An appraisal contains the risk classification, every expression outcome, point
totals, and optional weighted score state. Required-gate failure and total
weighted-evaluation failure are distinct states even though neither has a score.

## Reporting and rejection

Report generators consume the same crate, metric, and appraisal model to produce
console, JSON, HTML, CSV, or Excel output. Structured formats retain typed metric
values. JSON also exposes structured appraisal state and individual outcomes;
legacy display fields remain for compatibility.

CSV neutralizes formula-like untrusted text before escaping it. Excel emits
textual values as string cells. Numeric values remain numeric in both formats.

Risk thresholds can turn an appraisal into a command failure. The rejection
error identifies the affected crates and distinguishes failed policy requirements
from inconclusive evaluation. Its detail list is bounded to keep errors usable:
up to 20 crates and 10 non-passing outcomes per crate. When details are omitted,
the output points users to complete console or JSON reports.

Policy descriptions state the condition an expression expects. Rejection output
labels that text as `expected` rather than presenting the desired condition as
though it were the reason for rejection. Inconclusive output separately identifies
the expected condition and the evaluation error.

Allow-list entries are applied after appraisal. An allowed crate remains visible
with its computed metrics and risk, but it does not cause the command to fail.
