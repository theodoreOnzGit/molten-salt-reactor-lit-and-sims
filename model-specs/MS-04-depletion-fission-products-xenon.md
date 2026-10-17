# MS-04: Depletion, fission products, xenon and off-gas

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #46 (part of epic #42)

## Purpose

Track the fuel-salt inventory over power operation: uranium burn-up and
breeding, samarium and other poisons, xenon including its removal to the
off-gas and its migration into graphite, noble-metal plating and the
radioactivity that sets decay heat (MS-14) and source term.

## Scope

MSRE ²³⁵U and ²³³U power runs. Out of scope: chemical processing of the
fuel (fluorination, distillation), which ORNL-TM-2578 and ORNL-4577 describe
but which happened offline.

## Governing physics (on by default)

- Bateman equations with CRAM, one-group rates from MS-02 flux spectra,
  updated as composition changes.
- **Removal terms are physics, each on by default with its own ablation
  switch**: xenon stripping at the pump bowl spray, xenon diffusion into
  graphite, xenon transfer to circulating helium bubbles, noble-metal deposition
  on metal and graphite surfaces, noble-gas decay in the off-gas line.
- Xenon reactivity as a function of power and helium void fraction.
- Fission products in a circulating fuel decay in and out of the core; the
  inventory model must use the time-in-core fraction, not assume the fuel
  stays in the flux.

## Inputs and sources

| Input | Source |
|---|---|
| Xenon behaviour, measured poisoning vs power and void | ORNL-TM-3464 |
| Xenon migration model (1967) | ORNL-4069; ORNL-TM-1810 (short-lived noble gases into graphite) |
| Gas transport in MSRE graphite | ORNL-4148, ORNL-4389 |
| Bubble mass transfer | ORNL-TM-2245, ORNL-TM-3718 |
| Noble-metal migration | ORNL-TM-3884 |
| Fission product behaviour, measured | ORNL-4865; ORNL-TM-3151 (gamma spectrometry) |
| Reactivity balance over operation | ORNL-TM-1796 |
| Calculated fuel radioactivity | ORNL-TM-2970 (code-to-code reference only) |
| Decay and yield data | ENDF/B-VIII.0 |

## OUTRAM PARK starting point

- `outram-park-fork-onix` (unpublished): Bateman matrix and CRAM16, verified
  against closed-form Bateman (~1e-14). No nuclide libraries yet.
- `outram-mc-libs` `src/depletion/`: CRAM16, chain, predictor burn-up.
- `teh-o-prke`: Xe-135/I-135 constants and concentration solver (stationary
  fuel).
- `boon-lay`: Lagrangian decay and diffusion, but TRISO-only.
- No salt-specific removal pathway exists anywhere. New physics.

## Verification

- CRAM vs closed-form Bateman for short chains (reuse `onix` tests).
- Xenon removal model at steady state vs its analytical balance
  (production = decay + burn-out + stripping + graphite uptake).
- Conservation: every atom removed from the salt appears in a sink inventory.

## Validation

N-6 (#7) reactivity balance; N-7 (#8) xenon poisoning vs power and void;
fission-product distribution (salt, graphite, metal, off-gas) against
ORNL-4865 and ORNL-TM-3151. That last comparison has no benchmark issue yet.
Two ORNL-TM-3151 tables (off-gas jumper activities, fuel-line relative
activities) are already transcribed; ORNL-4865 is not. The report itself urges
restraint in using the fuel-line relative activities.

## Known traps

- **Mass-transfer coefficients are the classic calibration knobs.** Derive
  each from a correlation (ORNL-TM-3718, ORNL-TM-2245) and run uncalibrated
  first. Any calibration then follows AGENTS.md §2, with ablation.
- **Void fraction couples xenon to hydraulics.** Use the measured void
  (T-4, ORNL-TM-2315, ORNL-TM-2318) as the condition, not as a fitted output.
- **The 1960s ORNL xenon predictions are model output**, not measurement.
