# MS-16: Uncertainty quantification and sensitivity

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #58 (part of epic #42)

## Purpose

Give every V&V comparison an uncertainty on the prediction, so that a
disagreement with measurement can be judged. AGENTS.md §4 requires results
"with uncertainty"; this spec says how they are computed.

## Scope

Propagation of input uncertainty (properties, geometry, nuclear data,
correlations, compositions) through MS-01 to MS-15, and sensitivity ranking.
Not in scope: Bayesian calibration (it would follow the calibration rules in
AGENTS.md §2 if ever used).

## Method

- Each input carries a distribution taken from its source's stated
  uncertainty. Where a source gives none, say so and state the assumed
  distribution and why. Never shrink an uncertainty to make a comparison pass.
- Sampling (Latin hypercube or Monte Carlo) through the model; sample count
  set by convergence of the quantity compared.
- Sobol first-order and total indices to rank which inputs dominate each
  comparison.
- Nuclear-data uncertainty from ENDF covariances processed by ERRORR/COVR.

## Inputs and sources

Uncertainties as stated in each source cited by MS-01 to MS-15, and the
measurement uncertainties recorded in the
`validation-data/ai-generated/` CSV headers.

## OUTRAM PARK starting point

- `raffles`: nine distributions, Monte Carlo, Latin hypercube and grid
  samplers, Pearson and Spearman correlation, Sobol indices
  (`src/distributions.rs`, `src/samplers.rs`, `src/sensitivity.rs`, with
  tests). Its README still says "scaffold only", which its own source
  contradicts (an upstream doc defect). Surrogates are a placeholder.
- `njoy-outram-park-fork` ERRORR and COVR, compared against NJOY2016.

## Verification

- Sampler moments converge to the analytical moments of the distribution.
- Sobol indices of the Ishigami function match the analytical values.
- Linear model: sampled output variance equals the analytical sum.

## Validation

Not applicable directly; UQ supplies the error bars every other validation
uses.

## Known traps

- **Measurement and prediction uncertainties are separate.** Combine them
  only for the comparison, and show each.
- **Correlated inputs.** Properties of one salt from one source are
  correlated; sample them jointly where the source allows.
