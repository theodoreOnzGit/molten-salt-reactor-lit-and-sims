# MS-12: Graphite moderator

> **Status, 2026-10-17: specification only, nothing modelled.** AI-drafted,
> not human-reviewed. Follows [`AGENTS.md`](../AGENTS.md).

Tracking issue: #54 (part of epic #42)

## Purpose

Represent the graphite moderator's thermal properties, its irradiation
dimensional change and property degradation, salt and gas penetration into its
pores, and the fission products it absorbs. Graphite life limits core life in
a graphite-moderated MSR, and graphite temperature is a reactivity feedback.

## Scope

MSRE CGB graphite stringers; MSBR graphite as a design extension. Out of
scope: graphite oxidation in air ingress.

## Governing physics (on by default)

- Temperature-dependent conductivity, heat capacity and expansion, with the
  conductivity loss under irradiation.
- Fast-fluence dimensional change (shrinkage then turnaround and growth) and
  irradiation creep.
- Gas transport in pores (xenon uptake, MS-04) by diffusion and permeation.
- Salt intrusion as a function of pore entrance size and pressure.

## Inputs and sources

| Input | Source |
|---|---|
| Graphite behaviour and effects on MSBR performance | ORNL-TM-2136 |
| Gas transport in MSRE graphite | ORNL-4148, ORNL-4389 |
| Short-lived noble gases into graphite | ORNL-TM-1810 |
| Lithium and fluorine in graphite (salt penetration measurement) | ORNL-TM-2238 |
| MSRE graphite in the PIE | ORNL-TM-4174 |
| Graphite properties used in the 1960s nuclear analysis | ORNL-TM-0730 |
| Irradiation specimen array | ORNL-TM-2743 |

## OUTRAM PARK starting point

`tuas_boussinesq_solver` has nuclear graphite as a solid material for heat
conduction. No irradiation, gas or salt penetration model exists. The INL VTB
graphite model (infiltration, wear, stress, failure) in the backend's
`reference-data/virtual_test_bed/msr/` is a possible code-to-code reference,
not verified by OUTRAM PARK.

## Verification

- Conduction in a stringer vs the analytical solution with uniform heating.
- Pore diffusion vs the analytical slab uptake solution.

## Validation

Xenon uptake through N-7 (#8) and MS-04; salt penetration against
ORNL-TM-2238 and ORNL-TM-4174. **No graphite benchmark issue exists yet;
this spec's issue tracks filing one.** The MSRE graphite reached low fluence,
so irradiation dimensional change has no MSRE validation data in
`literature/`; state that gap rather than borrowing data of unknown licence.

## Known traps

- **Graphite grade matters.** Properties of one grade do not transfer to
  another.
- **The xenon diffusivity into graphite is a calibration knob** in MS-04;
  take it from ORNL-4148/4389 measurements, not from fitting N-7.
